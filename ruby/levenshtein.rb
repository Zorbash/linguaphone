module Levenshtein
  def self.distance(first_string, second_string)
    matrix = [(0..first_string.size).to_a]

    (1..second_string.size).each do |m|
      matrix << [m] + [0] * first_string.size
    end

    (1..second_string.size).each do |i|
      (1..first_string.size).each do |j|
        if first_string[j - 1] == second_string[i - 1]
          matrix[i][j] = matrix[i - 1][j - 1]
        else
          matrix[i][j] = calculate_matrix(matrix, i, j)
        end
      end
    end

    return matrix.last.last
  end

  def self.calculate_matrix(matrix, i, j)
    [matrix[i - 1][j], matrix[i][j - 1], matrix[i - 1][j - 1]].min + 1
  end
end
