def set_gvar(name, value):
    if type(value) is str:
        set_gvar_str(name, value)
    elif type(value) is int:
        set_gvar_int(name, value)
    else:
        raise TypeError('invalid type "' + type(value).__name__ + '" for set_gvar')
