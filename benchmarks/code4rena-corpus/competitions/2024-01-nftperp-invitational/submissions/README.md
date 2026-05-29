# Submissions for 2024-01-nftperp-invitational

Submission capture requires a logged-in Code4rena browser session.

Authenticated CSV endpoint:

```text
https://code4rena.com/api/v1/audits/HoiHTi8drSo/submissions/csv
```

After downloading the CSV from a logged-in session, run:

```bash
python3 scripts/archive_code4rena_corpus.py import-submissions-csv \
  --out benchmarks/code4rena-corpus \
  --slug 2024-01-nftperp-invitational \
  --csv /path/to/submissions.csv
```
