/**
 * swipe.js
 *
 * Horizontal swipe navigation for the app shell.
 *
 * Swipe right  → open filters sidebar  (or close settings if open)
 * Swipe left   → open settings sidebar (or close filters if open)
 *
 * Communicates with Dioxus state by clicking hidden control buttons
 * injected by shell.rs (ids: swipe-open-filters, swipe-close-filters,
 * swipe-open-settings, swipe-close-settings).
 */
(function () {
  'use strict';

  var THRESHOLD = 55;   // minimum horizontal px for swipe recognition
  var MAX_VERT  = 80;   // abort if vertical movement exceeds this (scrolling)

  var startX = 0;
  var startY = 0;
  var tracking = false;

  document.addEventListener('touchstart', function (e) {
    // Ignore touches that begin on interactive elements so swipe doesn't
    // accidentally fire when the user taps a button inside a sidebar.
    var tag = e.target.tagName.toLowerCase();
    if (tag === 'input' || tag === 'button' || tag === 'select' ||
        tag === 'textarea' || tag === 'a' || tag === 'label') {
      tracking = false;
      return;
    }
    startX   = e.touches[0].clientX;
    startY   = e.touches[0].clientY;
    tracking = true;
  }, { passive: true });

  document.addEventListener('touchend', function (e) {
    if (!tracking) return;
    tracking = false;

    var dx = e.changedTouches[0].clientX - startX;
    var dy = e.changedTouches[0].clientY - startY;

    if (Math.abs(dx) < THRESHOLD || Math.abs(dy) > MAX_VERT) return;

    var shell = document.querySelector('.app-shell');
    if (!shell) return;

    var filtersOpen  = shell.getAttribute('data-filters-open')  === 'true';
    var settingsOpen = shell.getAttribute('data-settings-open') === 'true';

    if (dx > 0) {
      // Swipe right
      if (settingsOpen) {
        click('swipe-close-settings');
      } else if (!filtersOpen) {
        click('swipe-open-filters');
      }
    } else {
      // Swipe left
      if (filtersOpen) {
        click('swipe-close-filters');
      } else if (!settingsOpen) {
        click('swipe-open-settings');
      }
    }
  }, { passive: true });

  function click(id) {
    var el = document.getElementById(id);
    if (el) el.click();
  }
})();
