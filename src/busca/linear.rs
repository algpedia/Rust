pub fn linear(_lista: &[i32], alvo: &i32) -> i32 {
  // Verifica todos os elementos
  for (index, valor) in _lista.iter().enumerate() {
    // Se o valor da posição for igual ao alvo, retorna o indíce
    if valor == alvo {
      return index as i32;
    }
  }

  // Caso valor não tenha sido encontrado, retorna -1
  return -1;
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn primeiro_index() {
    assert_eq!(linear(&[2, 1, 3, 7, 1, 5], &2), 0);
  }

  #[test]
  fn ultimo_index() {
    assert_eq!(linear(&[2, 1, 3, 7, 1, 5], &5), 5);
  }

  #[test]
  fn meio_index() {
    assert_eq!(linear(&[2, 1, 3, 7, 1, 5], &3), 2);
  }

  #[test]
  fn nao_encontrado() {
    assert_eq!(linear(&[2, 1, 3, 7, 1, 5], &77), -1);
  }
}
