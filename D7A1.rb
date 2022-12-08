deptharray = [[]]

currentDepth = 0

File.readlines('test.txt').each do |line|

   arr = line.chomp.split


   if arr[0] == '$'
    if arr[1] == 'cd'
      if arr[2] == '/'

    puts "true"
   end
    # case line.chomp
    # when "$ cd /"
    #     currentDepth = 0
        
    # when "$ cd \w"
    #     currentDepth += 1
    #     puts "swag"

    # when "$ cd .."
    #     currentDepth -= 1
        
    # when "$ ls"
        
    # end




end

puts currentDepth