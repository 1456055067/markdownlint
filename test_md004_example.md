# MD004 Test Examples

## Consistent Style (Default) - Valid
* Item 1
* Item 2
* Item 3

## Consistent Style (Default) - Invalid
* Item 1
- Item 2
+ Item 3

## Asterisk Style - Valid
* Item 1
* Item 2
* Item 3

## Dash Style - Valid
- Item 1
- Item 2
- Item 3

## Plus Style - Valid
+ Item 1
+ Item 2
+ Item 3

## Sublist Style - Valid
* Parent item 1
  + Child item 1
  + Child item 2
* Parent item 2
  - Child item 3

## Sublist Style - Invalid (same marker as parent)
* Parent item 1
  * Child item 1 (should be different)
  * Child item 2
