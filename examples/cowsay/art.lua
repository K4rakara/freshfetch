local cowsay = assert( io.popen("cowsay Death to windows") )

print ""

for line in cowsay:lines() do
	print(line)
end

print ""

cowsay:close()

