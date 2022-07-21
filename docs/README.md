# Teste
- Array com 1.000.000 inteiros (i32)
- Resultado é a média de 10 testes

# Solução multi-thread:
1. Divide o array em diferentes chunks (ou slices), um por thread.
2. Cada thread aplica o merge sort serial no seu chunk
3. Quando todas as threads terminarem, é feito um último merge entre as chunks
