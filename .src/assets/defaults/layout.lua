print(""..save())

print("  "..string.gsub(art, "\n", "\n  "))

print(""..up(artHeight))

print(""
	..right(artWidth + 4)
	..string.gsub(info,
		"\n",
		"\n"..right(artWidth + 4)))

print(""..restore())

if infoHeight > artHeight then
	print(""..down(infoHeight))
else
	print(""..down(artHeight))
end

