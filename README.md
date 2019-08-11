# sq

`sq` is a shell toolchain for helping you with text sequence labeling tasks. It can:

 * add annotations to a text using regular expressions;
 * build a templates for a machine learning pipeline.

## Usage

`sq` works with sample in a ndjson format. Suppose you have following file:

```
Please send me email to: ask@server.com
Reply to me@gmaiul.com ASAP
```

you can import this file in ndjson format:

```
sq import examples.txt > examples.ndjson 
```

```
{"text": "Please send me email to: ask@server.com"}
{"text": "Reply to me@gmaiul.com ASAP"}
```

If you want to mark all emails in a dataset, you can run (pardon oversimplified regexp):

```
$ sq mark -f dataset.ndjson -l email -r '[a-z0-9]@[a-z0-9\\.]'
```

then you will have:

```
{"text": "Please send me email to: ask@server.com", "annotations": [{"span": [25, 39], "label": "email"}]}
{"text": "Reply to me@gmaiul.com ASAP", "annotations": [{"span": [9, 23], "label": "email"}]}
```

If you want only print matches:

```
$ sq print -f examples.ndjson -r '[a-z0-9]@[a-z0-9\\.]'
```

Then you can mask given spans from a dataset:

```
$ seq mask -f dataset.ndjson -l email -c '*'
"Please send me email to: **************"
"Reply to ************ ASAP"
```
