# V12 Duplicate Checklist

```text
Same root cause as V12?
├─ NO → Submit
└─ YES → Same impact?
    ├─ NO → Submit
    └─ YES → Materially different exploit, victim, value flow, or mitigation?
        ├─ YES → Submit only if the delta is clear in one sentence
        └─ NO → Exclude (known V12 / prior-finding overlap)
```
