arr = Array.new

priohash = Hash.new
priohash = {"a"=>1,"b"=>2,"c"=>3,"d"=>4,"e"=>5,"f"=>6,"g"=>7,"h"=>8,"i"=>9,"j"=>10,"k"=>11,"l"=>12,"m"=>13,"n"=>14,"o"=>15,"p"=>16,"q"=>17,"r"=>18,"s"=>19,"t"=>20,"u"=>21,"v"=>22,"w"=>23,"x"=>24,"y"=>25,"z"=>26,
            "A"=>27,"B"=>28,"C"=>29,"D"=>30,"E"=>31,"F"=>32,"G"=>33,"H"=>34,"I"=>35,"J"=>36,"K"=>37,"L"=>38,"M"=>39,"N"=>40,"O"=>41,"P"=>42,"Q"=>43,"R"=>44,"S"=>45,"T"=>46,"U"=>47,"V"=>48,"W"=>49,"X"=>50,"Y"=>51,"Z"=>52}

elve1 = ""
elve2 = ""
elve3 = ""
i = 0

File.readlines('input3.txt').each do |line|
    if i == 0 
        elve1 = line.to_s
    end

    if i == 1
        elve2 = line.to_s
    end

    if i == 2
        elve3 = line.to_s

        puts elve1 + elve2 + elve3

        elve1.each_char do |n|
            if elve2.chars.include? n.to_s

                if elve3.chars.include? n.to_s
                    #puts "ture"
                    arr.append(priohash[n.to_s])
                    break
                end
            end
        end

        i = -1
    end 
    i += 1
end

puts arr.sum