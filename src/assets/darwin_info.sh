#!/usr/bin/sh
darwin_name="";
osx_version="";
osx_build="";
IFS=$'\n' read -d "" -ra sw_vers <<< "$(awk -F'<|>' '/key|string/ {print $3}' \
    "/System/Library/CoreServices/SystemVersion.plist")";
for ((i=0;i<${#sw_vers[@]};i+=2)) {
	case ${sw_vers[i]} in
		ProductName)          darwin_name=${sw_vers[i+1]} ;;
		ProductVersion)       osx_version=${sw_vers[i+1]} ;;
		ProductBuildVersion)  osx_build=${sw_vers[i+1]}   ;;
	esac
}
echo $darwin_name;
echo $osx_version;
echo $osx_build;