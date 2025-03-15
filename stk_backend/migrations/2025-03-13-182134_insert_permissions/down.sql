DELETE FROM permission WHERE name IN (
    'CREATE',
    'UPDATE',
    'DELETE',
    'ASSIGN_ROLE',
);