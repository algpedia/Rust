pub fn binaria(_lista: &[i32], alvo: &i32) -> i32 {
  return binaria_recursiva(_lista, 0, _lista.len(), alvo);
}

fn binaria_recursiva(_lista: &[i32], inicio: usize, fim: usize, alvo: &i32) -> i32 {
  if inicio >= fim {
    return -1;
  }

  let meio: usize = (inicio + fim) / 2;

  if _lista[meio] == *alvo {
    return meio as i32;
  } else if _lista[meio] > *alvo {
    return binaria_recursiva(_lista, inicio, meio, alvo);
  } else {
    return binaria_recursiva(_lista, meio + 1, fim, alvo);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn primeiro_index() {
    assert_eq!(binaria(&[1, 2, 3, 4, 5, 6, 7, 8], &1), 0);
  }

  #[test]
  fn ultimo_index() {
    assert_eq!(binaria(&[1, 2, 3, 4, 5, 6, 7, 8], &8), 7);
  }

  #[test]
  fn meio_index() {
    assert_eq!(binaria(&[1, 2, 3, 4, 5, 6, 7, 8], &3), 2);
  }

  #[test]
  fn nao_encontrado() {
    assert_eq!(binaria(&[1, 2, 3, 4, 5, 6, 7, 8], &77), -1);
  }
}