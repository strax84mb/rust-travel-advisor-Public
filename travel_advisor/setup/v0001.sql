CREATE DATABASE rust_travel_advisor
    CHARACTER SET utf8
    COLLATE utf8_general_ci;

CREATE USER 'rust_travel_adv_user' IDENTIFIED BY 'rust_travel_adv_pass';
CREATE USER 'rust_travel_admin' IDENTIFIED BY 'rust_travel_adm_pass';

GRANT SELECT, UPDATE, INSERT, DELETE ON rust_travel_advisor.* TO 'rust_travel_adv_user';
GRANT ALL PRIVILEGES ON rust_travel_advisor.* TO 'rust_travel_admin';

-- mysql://rust_travel_adv_user:rust_travel_adv_pass@localhost:3306/rust_travel_advisor
-- mysql://rust_travel_admin:rust_travel_adm_pass@localhost:3306/rust_travel_advisor
