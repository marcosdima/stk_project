DELETE FROM role_permission 
    WHERE role_id = (SELECT id FROM role WHERE name = 'Admin');
