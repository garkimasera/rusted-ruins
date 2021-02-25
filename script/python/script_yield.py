def _g():
    yield 1


_GeneratorType = type(_g())


class ScriptYield:
    def talk(text_id, choices=[]):
        return {
            "tag": "Talk",
            "talk": {
                "text_id": text_id,
                "choices": choices,
            },
        }

    def shop_buy():
        return {"tag": "ShopBuy"}

    def shop_sell():
        return {"tag": "ShopSell"}

    def quest():
        return {"tag": "Quest"}


def _get_next_script_yield():
    if not isinstance(_rrscript_gen, _GeneratorType):
        return None
    try:
        return _rrscript_gen.__next__()
    except StopIteration:
        return None
