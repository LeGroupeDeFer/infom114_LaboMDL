-- Your SQL goes here
CREATE TABLE roles_capabilities (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  role_id INT UNSIGNED NOT NULL,
  capability_id INT UNSIGNED NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT unique_role_capability_rolescapabilities UNIQUE (role_id, capability_id),
  FOREIGN KEY idx_fk_role_rolescapabilities(role_id) REFERENCES roles(id),
  FOREIGN KEY idx_fk_capability_rolescapabilities (capability_id) REFERENCES capabilities(id)
);