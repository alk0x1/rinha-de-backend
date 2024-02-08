CREATE TABLE clientes (
  nome VARCHAR(255) NOT NULL,
  limite INT NOT NULL,
  saldo INT NOT NULL
);

DO $$
BEGIN
  INSERT INTO clientes (nome, limite, saldo)
  VALUES
    ('Rodrigo', 1000 * 100, 0),
    ('Reinaldo', 800 * 100, 0),
    ('Rafael', 10000 * 100, 0),
    ('Rosana', 100000 * 100, 0),
    ('Roseli', 5000 * 100, 0);
END; $$
