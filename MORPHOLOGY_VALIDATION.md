# Morphology Validation Report
*Generated: 2026-04-16*

## Summary
- Total forms checked: 5737
- Found in dictionary: 725 (12.6%)
- Issues identified: ~46 potential

## Issues Fixed During Validation

| Form | Lemma | DB Tense | Should Be | Fix Applied |
|------|-------|----------|-----------|--------------|
| στῆσον | ἵστημι | fut | aor1 | ✅ Fixed |
| ἕστηκα/ἕστηκας/ἕστηκε | ἵστημι | fut | perf | ✅ Fixed |
| ὦ, ᾖς, ᾖ | ἵστημι | fut | perf | ✅ Fixed |

## Known Issues (Not Fixed - Require Manual Review)

### 1. ἵστημι Present vs Imperfect Ambiguity
Forms like ἵστης, ἵσταμεν, ἵστατε are marked as pres in DB but dictionary shows imperf.

**Reason**: ἵστημι can mean "I make stand" (transitive - aorist) or "I stand" (intransitive - present). The dictionary treats some forms as imperfect.

**Status**: DB might be correct - these are present forms, just can have imperfect meaning in certain contexts.

### 2. εἴην Ambiguity
The form εἴην (ID 3907) is marked as fut|act|opt for ἵστημι, but:
- Dictionary shows it as present opt of εἰμί
- It's actually a perfect active optative form of ἵστημι (εἱστηκώς)

**Status**: Unclear - needs grammar reference check

### 3. Forms Not in Dictionary
~5000 forms (87.4%) not found in gr-en dictionary. This is expected for:
- Forms with different diacritical marks
- Forms that are simply not in this particular dictionary
- Variant spellings

## Forms Requiring Manual Review

These forms have conflicting/missing analysis that couldn't be automatically resolved:

```
ἵστης - DB: pres, Dict: imperf
ἵσταμεν - DB: pres, Dict: imperf  
ἵστατε - DB: pres, Dict: imperf
ἵστη - DB: pres, Dict: imperf
ἵστασθε - DB: pres, Dict: imperf
εἴην - DB: fut, Dict: pres (for εἰμί)
ἥσω - DB: fut, Dict: aor (epic)
ἥσεις - DB: fut, Dict: noun (fem)
```

## Recommendations

1. **Manual review** of forms marked with "Requires Manual Review" above
2. **Add more dictionary entries** - the 12.6% coverage is low
3. **Consider adding grammar-smyth-master** as additional reference for complex verbs like ἵστημι, ἵημι