
stack = [["B","G","S","C"],
         ["T","M","W","H","J","N","V","G"],
         ["M","Q","S"],
         ["B","S","L","T","W","N","M"],
         ["J","Z","F","T","V","G","W","P"],
         ["C","T","B","G","Q","H","S"],
         ["T","J","P","B","W"],
         ["G","D","C","Z","F","T","Q","M"],
         ["N","S","H","B","P","F"]]



File.readlines('input5.txt').each do |line|

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


