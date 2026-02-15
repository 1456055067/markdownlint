# MD003 Test Examples

This file demonstrates the different heading styles that MD003 checks.

## ATX Style Headings

```markdown
# Heading 1
## Heading 2
### Heading 3
```

## ATX Closed Style Headings

```markdown
# Heading 1 #
## Heading 2 ##
### Heading 3 ###
```

## Setext Style Headings

```markdown
Heading 1
=========

Heading 2
---------
```

## Mixed Styles (Would fail with "consistent" mode)

```markdown
# ATX Heading

Setext Heading
--------------
```

## Setext with ATX Mode

```markdown
Heading 1
=========

Heading 2
---------

### Heading 3
#### Heading 4
```
