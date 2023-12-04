import hook
from hook.imports.types import Frame, Context
class Hook(hook.Hook):
    def hook(self, frame: Frame, ctx: Context) -> str:
        return "Hello, World!"
