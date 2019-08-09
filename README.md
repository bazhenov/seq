# sq

`sq` is a shell toolchain for helping you with text sequence labeling tasks. It can:

 * add annotations to a text using regular expressions;
 * build a templates for a machine learning pipeline.

## Usage

`sq` works with sample in a ndjson format. Suppose you have following file:

```
{"text": "Please send me email to: ask@server.com"}
{"text": "Reply to me@gmaiul.com ASAP"}
```

If you want to mark all emails in a dataset, you can run (pardon oversimplified regexp):

```
$ sq -f dataset.ndjson mark -l email -r '[a-z0-9]@[a-z0-9\\.]'
```

then you will have:

```
{"text": "Please send me email to: ask@server.com", "annotations": [{"span": [25, 39], "label": "email"}]}
{"text": "Reply to me@gmaiul.com ASAP", "annotations": [{"span": [9, 23], "label": "email"}]}
```

Then you can mask given spans from a dataset:

```
$ seq -f dataset.ndjson mask -l email -c '*'
"Please send me email to: **************"
"Reply to ************ ASAP"
```
