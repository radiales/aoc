score = Array.new

File.readlines('input2.txt').each do |line|
  opponent = line[0]
  me = line[2]

  #X A Rock
  #Y B Paper
  #Z C Scissor

  case me
    when "X" # Rock
      s1 = 1

      case opponent
      when "A" # Rock
        s2 = 3
      when "B" # Paper
        s2 = 0
      when "C" # Scissor
        s2 = 6
      end

    when "Y" # Paper
      s1 = 2
      case opponent
      when "A" # Rock
        s2 = 6
      when "B" # Paper
        s2 = 3
      when "C" # Scissor
        s2 = 0
      end

      when "Z" # Scissor
      s1 = 3
      case opponent
      when "A" # Rock
        s2 = 0
      when "B" # Paper
        s2 = 6
      when "C" # Scissor
        s2 = 3
      end
  end

  erg = s1 + s2
  score.append(erg)
  s1 = 0
  s2 = 0
  sum = 0
end

puts(score.sum)
