
stack = [   
["Z","N"],
["M","C","D"],
["P"],
]



File.readlines('test.txt').each do |line|

amount = line.scan(/\d+/)[0]
origin = line.scan(/\d+/)[1]
destin = line.scan(/\d+/).last


tmp = []

amount.to_i.times do

        tmp.push(stack[origin.to_i-1].pop) # Gets the top of the stack (-1 to adjust for array index)
        stack[destin.to_i-1].push(tmp.pop) # Pushes contents to new destination
end

end

stack.each do |x|
puts x.pop
end


#FAIL FSZWBPTCG