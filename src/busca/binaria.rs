pub fn binaria(_lista: &[i32], alvo: &i32) -> i32 {
  return binaria_recursiva(_lista, 0, _lista.len(), alvo);
}

fn binaria_recursiva(_lista: &[i32], inicio: usize, fim: usize, alvo: &i32) -> i32 {
  // Se os ponteiros de inicio e fim colidirem, significa que estamos buscando em um intervalo vazio. 
  // Logo o alvo não está na lista
  if inicio >= fim {
    return -1;
  }

  // Salva o ponto do meio da lista
  let meio: usize = (inicio + fim) / 2;

  // Define os limites da busca dependendo se o alvo for igual, menor ou maior que o ponto do meio atual
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