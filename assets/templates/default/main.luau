--?ScriptVersion
local _version = ""

function onTransitionBegin(transitionInfo: Transition)
	alwaysTick = true
end

function main() end

function onGui() end

function tick(dt)
	if not camera.isActive and not camera.isFollowed then
		if alwaysTick then
			alwaysTick = false
			onExit()
		end
		return
	end
end

function onExit() end
