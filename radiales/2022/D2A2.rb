score = Array.new

File.readlines('input2.txt').each do |line|
  opponent = line[0]
  me = line[2]

  #X A Rock
  #Y B Paper
  #Z C Scissor

  case me
  when "X" # Loose
    case opponent
    when "A" # Rock
      s2 = 0
      s1 = 3
    when "B" # Paper
      s2 = 0
      s1 = 1
    when "C" # Scissor
      s2 = 0
      s1 = 2
    end

  when "Y" # Draw
    case opponent
    when "A" # Rock
      s2 = 3
      s1 = 1
    when "B" # Paper
      s2 = 3
      s1 = 2
    when "C" # Scissor
      s2 = 3
      s1 = 3
    end

  when "Z" # Win
    case opponent
    when "A" # Rock
      s2 = 6
      s1 = 2
    when "B" # Paper
      s2 = 6
      s1 = 3
    when "C" # Scissor
      s2 = 6
      s1 = 1
    end
  end

  erg = s1 + s2
  score.append(erg)
  s1 = 0
  s2 = 0
  sum = 0
end

puts(score.sum)