class ScriptYield:
    def quest():
        return {
            'tag': 'Quest'
        }

def _get_next_script_yield():
    try:
        return _rrscript_gen.__next__()
    except StopIteration:
        return None
