import numpy as np
from .configuration import SatType

from abc import (
    ABC,
    abstractmethod,
)


class Satellite(ABC):

    @property
    @abstractmethod
    def position(self) -> np.ndarray:
        ...

    @property
    @abstractmethod
    def velocity(self) -> np.ndarray:
        ...

    @property
    @abstractmethod
    def mass(self) -> float:
        ...

    @property
    @abstractmethod
    def characteristic_length(self) -> float:
        ...

    @property
    @abstractmethod
    def type(self) -> SatType :
        ...
