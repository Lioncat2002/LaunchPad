launchpad:register_global_command("hello-world-command")

function handler(num)
    result={}
    result[1]="kittycat"
    result[2]="hello world"
    return result
end