INSERT INTO role_permission (role_id, permission_id)
    SELECT r.id, p.id
    FROM role r, permission p
    WHERE r.name = 'Admin'
    AND p.name IN ('CREATE', 'UPDATE', 'DELETE', 'ASSIGN_ROLE');
    