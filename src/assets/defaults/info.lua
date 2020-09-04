
-- user@host
if context ~= nil then
	print(""
		..bold()
		..distroColors[1]
		..context.user
		..reset()
		..bold()
		.."@"
		..distroColors[2]
		..context.host
		..reset())
end

-- OS
if distro ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."OS"
		..reset()
		..": "
		..distro.shortname
		.." "
		..distro.architecture)
end

-- Kernel
if kernel ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Kernel"
		..reset()
		..": "
		..kernel.name
		.." "
		..kernel.version)
end

-- Uptime
if uptime ~= nil then
	local output = ""
	local function comma()
		if output ~= "" then output = output..", " end
	end
	local function s(v)
		if v ~= 1 and v ~= 0 then
			return "s"
		else
			return ""
		end
	end
	if uptime.days >= 1 then
		output = output..uptime.days.." day"..s(uptime.days)
	end
	if uptime.hours >= 1 then
		comma()
		output = output..uptime.hours.." hour"..s(uptime.hours)
	end
	if uptime.minutes >= 1 then
		comma()
		output = output..uptime.minutes.." minute"..s(uptime.minutes)
	elseif uptime.hours == 0 then
		comma()
		output = output..uptime.seconds.." second"..s(uptime.seconds)
	end
	
	print(""
		..bold()
		..distroColors[2]
		.."Uptime"
		..reset()
		..": "
		..output)
end

-- Packages
if packageManagers ~= nil then
	local output = ""
	if #packageManagers ~= 0 then
		for i,packageManager in pairs(packageManagers) do
			if packageManager.packages == 0 then
				table.remove(packageManagers, i)
			end
		end
		for i,packageManager in pairs(packageManagers) do
			if i ~= #packageManagers then
				output = output
					..packageManager.packages
					.." ("
					..packageManager.name
					.."), "
			else
				output = output
					..packageManager.packages
					.." ("
					..packageManager.name
					..")"
			end
		end
	else
		output = "0"
	end
	print(""
		..bold()
		..distroColors[2]
		.."Packages"
		..reset()
		..": "
		..output)
end

-- Shell
if shell ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Shell"
		..reset()
		..": "
		..shell.name
		.." "
		..shell.version)
end

-- Resolution
if resolution ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."Resolution"
		..reset()
		..": "
		..resolution.width
		.."x"
		..resolution.height)
end

-- DE
if de ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."DE"
		..reset()
		..": "
		..de.name
		.." "
		..de.version)
end

-- WM
if wm ~= nil then
	print(""
		..bold()
		..distroColors[2]
		.."WM"
		..reset()
		..": "
		..wm)
end

-- CPU
if cpu ~= nil then
	local freq = (cpu.freq >= 1000)
		and ""..(cpu.freq / 1000).."GHz"
		or  ""..cpu.freq.."MHz"
	print(""
		..bold()
		..distroColors[2]
		.."CPU"
		..reset()
		..": "
		..cpu.name
		.." ("
		..cpu.cores
		..") @ "
		..freq)
end

-- GPU
if gpus ~= nil then
	if #gpus ~= 1 then
		print(""
			..bold()
			..distroColors[2]
			.."GPUs"
			..reset()
			..": ")
		for _,gpu in pairs(gpus) do
			print(" - "..gpu.brand.." "..gpu.name)
		end
	else
		print(""
			..bold()
			..distroColors[2]
			.."GPU"
			..reset()
			..": "
			..gpus[1].brand
			.." "
			..gpus[1].name)
	end
end

-- Memory
if memory ~= nil then
	-- This memory math is probably inaccurate, but idk how to make it right ;-;
	print(""
		..bold()
		..distroColors[2]
		.."Memory"
		..reset()
		..": "
		..math.floor(memory.used / 1024)
		.."MB / "
		..math.floor(memory.max / 1024)
		.."MB")
end

-- Palette
print("")
print(""
	..blackBg()  .."   "
	..redBg()    .."   "
	..greenBg()  .."   "
	..yellowBg() .."   "
	..blueBg()   .."   "
	..magentaBg().."   "
	..cyanBg()   .."   "
	..whiteBg()  .."   ")
print(""
	..blackBrightBg()  .."   "
	..redBrightBg()    .."   "
	..greenBrightBg()  .."   "
	..yellowBrightBg() .."   "
	..blueBrightBg()   .."   "
	..magentaBrightBg().."   "
	..cyanBrightBg()   .."   "
	..whiteBrightBg()  .."   ")
