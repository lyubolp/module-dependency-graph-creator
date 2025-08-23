# Case 1: import module [as ident] (, module [as ident])*
import os
import sys, math
import collections.abc
import numpy as np
import os, sys as system, collections.abc as abc
import grader.utils.constants as const


# Case 2: from relative_module import ident [as ident] (, ident [as ident])*
from math import pi
from os.path import join
from .utils import helper
from ..submodule import func as subfunc
from package.module import a, b, c
from mypkg.tools import a as aa, b, c as cc
from .. import something
from grader.utils import files
from grader.checks.checks_factory import create_checks
from grader.utils.config import load_config
from grader.utils.files import get_tests_directory_name
from grader.utils.virtual_environment import VirtualEnvironment


# Case 3: from relative_module import ( ident [as ident] (, ident [as ident])* [,] )
from math import sin, cos, tan
from .utils import a as aa, b, c as cc
from ..submodule import (
    func,
    helper as h,
)
from package.module import (
    x,
    y,
)
from .. import (
    something,
)

from grader.checks.abstract_check import (
    AbstractCheck,
    CheckError,
    NonScoredCheckResult,
    CheckResult,
    ScoredCheckResult,
    ScoredCheck,
    NonScoredCheck,
)

# Case 4: from relative_module import *
from math import *
from .utils import *
from ..submodule import *
from package.module import *
from .. import *
