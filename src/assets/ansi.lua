function reset() return "\x1b[0m" end

function bold() return "\x1b[1m" end

function dim() return "\x1b[2m" end

function red() return "\x1b[38;5;1m" end

function redBright() return "\x1b[38;5;9m" end

function yellow() return "\x1b[38;5;3m" end

function yellowBright() return "\x1b[38;5;11m" end

function green() return "\x1b[38;5;2m" end

function greenBright() return "\x1b[38;5;10m" end

function cyan() return "\x1b[38;5;6m" end

function cyanBright() return "\x1b[38;5;14m" end

function blue() return "\x1b[38;5;4m" end

function blueBright() return "\x1b[38;5;12m" end

function magenta() return "\x1b[38;5;5m" end

function magentaBright() return "\x1b[38;5;13m" end

function black() return "\x1b[38;5;0m" end

function blackBright() return "\x1b[38;5;8m" end

function white() return "\x1b[38;5;7m" end

function whiteBright() return "\x1b[38;5;15m" end

function redBg() return "\x1b[48;5;1m" end

function redBrightBg() return "\x1b[48;5;9m" end

function yellowBg() return "\x1b[48;5;3m" end

function yellowBrightBg() return "\x1b[48;5;11m" end

function greenBg() return "\x1b[48;5;2m" end

function greenBrightBg() return "\x1b[48;5;10m" end

function cyanBg() return "\x1b[48;5;6m" end

function cyanBrightBg() return "\x1b[48;5;14m" end

function blueBg() return "\x1b[48;5;4m" end

function blueBrightBg() return "\x1b[48;5;12m" end

function magentaBg() return "\x1b[48;5;5m" end

function magentaBrightBg() return "\x1b[48;5;13m" end

function blackBg() return "\x1b[48;5;0m" end

function blackBrightBg() return "\x1b[48;5;8m" end

function whiteBg() return "\x1b[48;5;7m" end

function whiteBrightBg() return "\x1b[48;5;15m" end

function up(a) return "\x1b["..a.."A" end

function down(a) return "\x1b["..a.."B" end

function left(a) return "\x1b["..a.."D" end

function right(a) return "\x1b["..a.."C" end

function save() return "\x1b[s" end

function restore() return "\x1b[u" end
