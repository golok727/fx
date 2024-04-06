# FX Grammer


Fx is a formula language


```
a = 10
b = 10

=>  a + b

```
```
priceColumn = @.get(priceColumn)
priceColumn.count() // number of items in the column
priceColumn.sum() // if it is a col of type number then returns the sum of values else throws error
... 

for item in priceColumn {
		print(item)
}

priceColumn.forEach(|item| {
			print(item)
})

tax = priceColumn.map(|price| {
		>>> price * 0.1
})

@.columns.add(tax, { unique = true })
@.columns.length
	
```

### Keywords
- `=>` The final result.
- `->` function return keyword.

