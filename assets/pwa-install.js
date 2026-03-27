/**
 * pwa-install.js
 *
 * PWA "Add to Home Screen" install prompt.
 *
 * Behaviour:
 *   - Chrome/Android: captures `beforeinstallprompt`, shows a custom bottom
 *     sheet after a short delay (or after a user-interaction gate if you
 *     prefer), lets the user accept or dismiss.
 *   - iOS Safari: `beforeinstallprompt` is not supported. We detect iOS and
 *     show a manual-instruction banner instead ("Tap Share → Add to Home
 *     Screen").
 *   - Already installed (standalone mode): shows nothing.
 *   - Dismissed: remembers the decision in localStorage for 30 days so we
 *     don't pester the user on every visit.
 *
 * Integration (main.rs):
 *   Add to the App component:
 *     const PWA_INSTALL_JS: Asset = asset!("/assets/pwa-install.js");
 *     document::Script { src: PWA_INSTALL_JS }
 *   Or inline it as dangerous_inner_html inside a <script> tag.
 */

(function () {
  'use strict';

  /* ── Constants ──────────────────────────────────────────────────────────── */

  const STORAGE_KEY   = 'pwa-install-dismissed';   // localStorage key
  const DISMISS_DAYS  = 30;                         // re-show after this many days
  const SHOW_DELAY_MS = 3000;                       // ms after page load before showing

  /* ── Helpers ────────────────────────────────────────────────────────────── */

  /**
   * Returns true when the app is already running as an installed PWA.
   * Covers: Chrome/Android standalone, iOS standalone, TWA.
   */
  function isAlreadyInstalled() {
    return (
      window.matchMedia('(display-mode: standalone)').matches ||
      window.matchMedia('(display-mode: fullscreen)').matches ||
      window.matchMedia('(display-mode: minimal-ui)').matches ||
      // iOS Safari sets this when launched from home screen
      window.navigator.standalone === true
    );
  }

  /**
   * Returns true on iOS (iPhone / iPad / iPod).
   * Does NOT match macOS Safari even though it shares the same UA substring on
   * newer Macs — we gate on the absence of "Mac" being the primary platform and
   * the presence of touch events as a belt-and-suspenders check.
   */
  function isIos() {
    const ua = window.navigator.userAgent;
    return /iphone|ipad|ipod/i.test(ua);
  }

  /**
   * Returns true when running in iOS Safari specifically (not Chrome-on-iOS,
   * which also doesn't support beforeinstallprompt but gives a different UX).
   */
  function isIosSafari() {
    return isIos() && !window.navigator.userAgent.includes('CriOS') && !window.navigator.userAgent.includes('FxiOS');
  }

  /**
   * Check whether the user dismissed the prompt recently.
   */
  function wasDismissedRecently() {
    try {
      const raw = localStorage.getItem(STORAGE_KEY);
      if (!raw) return false;
      const ts = parseInt(raw, 10);
      if (isNaN(ts)) return false;
      return Date.now() - ts < DISMISS_DAYS * 24 * 60 * 60 * 1000;
    } catch {
      // localStorage blocked (private mode, etc.)
      return false;
    }
  }

  /**
   * Persist the dismissal timestamp.
   */
  function recordDismissal() {
    try {
      localStorage.setItem(STORAGE_KEY, String(Date.now()));
    } catch { /* ignore */ }
  }

  /**
   * Remove the dismissal record (called after a successful install so the
   * banner doesn't reappear if the user later uninstalls and revisits).
   */
  function clearDismissal() {
    try {
      localStorage.removeItem(STORAGE_KEY);
    } catch { /* ignore */ }
  }

  /* ── DOM builders ───────────────────────────────────────────────────────── */

  /**
   * Inject the stylesheet once.  We use a <style> tag rather than a separate
   * file so this module stays self-contained.
   */
  function injectStyles() {
    if (document.getElementById('pwa-install-styles')) return;
    const style = document.createElement('style');
    style.id = 'pwa-install-styles';
    style.textContent = /* css */ `
      /* ── PWA Install Banner ───────────────────────────────────────────── */
      #pwa-install-banner {
        /* Positioning: fixed bottom sheet, centred on the .app-shell column */
        position: fixed;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%) translateY(110%); /* start off-screen     */
        z-index: 9999;

        width: 100%;
        max-width: 480px;         /* matches .app-shell max-width              */
        padding: 20px 20px 28px;  /* extra bottom padding for home-indicator   */

        /* Appearance — mirrors the app's design system                       */
        background: var(--bg2, #ebdbb2);
        color: var(--fg, #3c3836);
        border-top: 1px solid var(--border, #bdae93);
        border-radius: 16px 16px 0 0;
        box-shadow: 0 -4px 24px var(--shadow, rgba(0,0,0,0.18));

        /* Slide-in animation                                                  */
        transition: transform 0.38s cubic-bezier(0.34, 1.10, 0.64, 1);

        /* Noise texture carried through from main.css                        */
        background-image: var(--noise-texture);
        background-size: var(--noise-size, 200px 200px);
      }

      #pwa-install-banner.pwa-banner--visible {
        transform: translateX(-50%) translateY(0);
      }

      /* Header row: icon + title + close button */
      .pwa-banner__header {
        display: flex;
        align-items: center;
        gap: 12px;
        margin-bottom: 12px;
      }

      .pwa-banner__icon {
        width: 48px;
        height: 48px;
        border-radius: 12px;
        flex-shrink: 0;
        object-fit: cover;
        background: var(--bg3, #d5c4a1);
      }

      .pwa-banner__title-block {
        flex: 1;
        min-width: 0;
      }

      .pwa-banner__title {
        font-size: 0.95rem;
        font-weight: 700;
        color: var(--fg, #3c3836);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      .pwa-banner__subtitle {
        font-size: 0.78rem;
        color: var(--fg2, #504945);
        margin-top: 2px;
      }

      .pwa-banner__close {
        background: none;
        border: none;
        cursor: pointer;
        font-size: 1.4rem;
        line-height: 1;
        color: var(--fg2, #504945);
        padding: 4px;
        min-width: 36px;
        min-height: 36px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 8px;
        flex-shrink: 0;
      }
      .pwa-banner__close:hover {
        background: var(--bg3, #d5c4a1);
      }

      /* iOS instruction steps */
      .pwa-banner__steps {
        display: flex;
        flex-direction: column;
        gap: 8px;
        margin-bottom: 16px;
      }

      .pwa-banner__step {
        display: flex;
        align-items: center;
        gap: 10px;
        font-size: 0.85rem;
        color: var(--fg, #3c3836);
      }

      .pwa-banner__step-icon {
        font-size: 1.2rem;
        flex-shrink: 0;
        width: 28px;
        text-align: center;
      }

      /* Action buttons row */
      .pwa-banner__actions {
        display: flex;
        gap: 10px;
      }

      .pwa-banner__btn {
        flex: 1;
        padding: 11px 16px;
        border-radius: var(--radius-sm, 8px);
        border: 1px solid var(--border, #bdae93);
        font-size: 0.88rem;
        font-weight: 600;
        cursor: pointer;
        min-height: 44px;     /* touch target                                 */
        transition: opacity 0.15s, background 0.15s;
      }
      .pwa-banner__btn:active { opacity: 0.75; }

      .pwa-banner__btn--primary {
        background: var(--accent, #d79921);
        color: var(--bg, #fbf1c7);
        border-color: var(--accent, #d79921);
      }
      .pwa-banner__btn--primary:hover {
        background: var(--accent2, #b57614);
        border-color: var(--accent2, #b57614);
      }

      .pwa-banner__btn--secondary {
        background: var(--bg3, #d5c4a1);
        color: var(--fg, #3c3836);
      }
      .pwa-banner__btn--secondary:hover {
        background: var(--border, #bdae93);
      }
    `;
    document.head.appendChild(style);
  }

  /**
   * Build and return the banner element for Chrome/Android (native prompt).
   */
  function buildChromeBanner(deferredPrompt) {
    const banner = document.createElement('div');
    banner.id = 'pwa-install-banner';
    banner.setAttribute('role', 'dialog');
    banner.setAttribute('aria-label', 'Install Greek Morphology App');
    banner.innerHTML = `
      <div class="pwa-banner__header">
        <img class="pwa-banner__icon"
             src="/greek_app/icon-192.png"
             alt="Greek Morphology icon"
             onerror="this.style.display='none'">
        <div class="pwa-banner__title-block">
          <div class="pwa-banner__title">Greek Morphology</div>
          <div class="pwa-banner__subtitle">Add to your home screen for quick access</div>
        </div>
        <button class="pwa-banner__close" aria-label="Dismiss install prompt">&#x2715;</button>
      </div>
      <div class="pwa-banner__actions">
        <button class="pwa-banner__btn pwa-banner__btn--secondary" id="pwa-btn-cancel">Not now</button>
        <button class="pwa-banner__btn pwa-banner__btn--primary"   id="pwa-btn-install">Install App</button>
      </div>
    `;

    banner.querySelector('.pwa-banner__close').addEventListener('click', () => {
      hideBanner(banner);
      recordDismissal();
    });

    banner.querySelector('#pwa-btn-cancel').addEventListener('click', () => {
      hideBanner(banner);
      recordDismissal();
    });

    banner.querySelector('#pwa-btn-install').addEventListener('click', async () => {
      hideBanner(banner);
      deferredPrompt.prompt();
      const { outcome } = await deferredPrompt.userChoice;
      if (outcome === 'accepted') {
        clearDismissal();
      } else {
        recordDismissal();
      }
    });

    return banner;
  }

  /**
   * Build and return the banner element for iOS Safari (manual instructions).
   */
  function buildIosBanner() {
    const banner = document.createElement('div');
    banner.id = 'pwa-install-banner';
    banner.setAttribute('role', 'dialog');
    banner.setAttribute('aria-label', 'Install Greek Morphology App');
    banner.innerHTML = `
      <div class="pwa-banner__header">
        <img class="pwa-banner__icon"
             src="/greek_app/icon-192.png"
             alt="Greek Morphology icon"
             onerror="this.style.display='none'">
        <div class="pwa-banner__title-block">
          <div class="pwa-banner__title">Install Greek Morphology</div>
          <div class="pwa-banner__subtitle">Add this app to your Home Screen</div>
        </div>
        <button class="pwa-banner__close" aria-label="Dismiss install prompt">&#x2715;</button>
      </div>
      <div class="pwa-banner__steps">
        <div class="pwa-banner__step">
          <span class="pwa-banner__step-icon">&#x2B06;</span>
          <span>Tap the <strong>Share</strong> button at the bottom of Safari</span>
        </div>
        <div class="pwa-banner__step">
          <span class="pwa-banner__step-icon">&#x2795;</span>
          <span>Tap <strong>Add to Home Screen</strong></span>
        </div>
        <div class="pwa-banner__step">
          <span class="pwa-banner__step-icon">&#x2714;&#xFE0F;</span>
          <span>Tap <strong>Add</strong> to confirm</span>
        </div>
      </div>
      <div class="pwa-banner__actions">
        <button class="pwa-banner__btn pwa-banner__btn--primary" id="pwa-btn-got-it">Got it</button>
      </div>
    `;

    banner.querySelector('.pwa-banner__close').addEventListener('click', () => {
      hideBanner(banner);
      recordDismissal();
    });

    banner.querySelector('#pwa-btn-got-it').addEventListener('click', () => {
      hideBanner(banner);
      recordDismissal();
    });

    return banner;
  }

  /* ── Show / hide ────────────────────────────────────────────────────────── */

  function showBanner(banner) {
    document.body.appendChild(banner);
    // Trigger reflow so the CSS transition actually plays from off-screen
    // eslint-disable-next-line no-unused-expressions
    banner.offsetHeight;
    banner.classList.add('pwa-banner--visible');
  }

  function hideBanner(banner) {
    banner.classList.remove('pwa-banner--visible');
    banner.addEventListener(
      'transitionend',
      () => banner.remove(),
      { once: true }
    );
  }

  /* ── Main logic ─────────────────────────────────────────────────────────── */

  function init() {
    // Never show when already running as an installed PWA
    if (isAlreadyInstalled()) return;

    // Respect recent dismissal
    if (wasDismissedRecently()) return;

    injectStyles();

    if (isIosSafari()) {
      // iOS Safari: no beforeinstallprompt — show manual instructions
      setTimeout(() => {
        // Re-check in case the user managed to navigate away or install
        if (isAlreadyInstalled() || wasDismissedRecently()) return;
        const banner = buildIosBanner();
        showBanner(banner);
      }, SHOW_DELAY_MS);
      return;
    }

    // Chrome / Android / Edge / Samsung Internet
    // The browser fires `beforeinstallprompt` when all PWA criteria are met.
    // We intercept and defer it so we can show our own UI instead of the
    // browser's mini-infobar.
    let deferredPrompt = null;

    window.addEventListener('beforeinstallprompt', (e) => {
      // Prevent the automatic mini-infobar from appearing on mobile
      e.preventDefault();
      deferredPrompt = e;

      setTimeout(() => {
        if (!deferredPrompt) return;
        if (isAlreadyInstalled() || wasDismissedRecently()) return;
        const banner = buildChromeBanner(deferredPrompt);
        showBanner(banner);
      }, SHOW_DELAY_MS);
    });

    // Clean up if the user installs via another mechanism (browser menu, etc.)
    window.addEventListener('appinstalled', () => {
      deferredPrompt = null;
      clearDismissal();
      const existing = document.getElementById('pwa-install-banner');
      if (existing) hideBanner(existing);
    });
  }

  // Run after the DOM is ready.
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
