print(save())

print(down(math.floor((artHeight / 8)) - 2))

print("  "..string.gsub(art, "\n", "\n  "))

print(up(artHeight + math.floor(artHeight / 4)))

print(""
	..right(artWidth + 4)
	..string.gsub(info,
		"\n",
		"\n"..right(artWidth + 4)))

print(restore())

print(""..down(infoHeight - 1))

