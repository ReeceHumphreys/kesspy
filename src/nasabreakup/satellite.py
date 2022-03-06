from abc import (
    ABC,
    abstractmethod,
)


class Satellite(ABC):

    @property
    @abstractmethod
    def position(self):
        ...

    @property
    @abstractmethod
    def velocity(self):
        ...

    @property
    @abstractmethod
    def mass(self):
        ...

    @property
    @abstractmethod
    def characteristic_length(self):
        ...

    @property
    @abstractmethod
    def type(self):
        ...
