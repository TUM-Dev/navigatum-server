import typing
from pathlib import Path

import yaml
from external.models import roomfinder
from external.models.common import PydanticConfiguration
from PIL import Image

BASE = Path(__file__).parent.parent.parent
EXTERNAL_RESULTS_PATH = BASE / "external" / "results"
SOURCES_PATH = BASE / "sources"
CUSTOM_RF_DIR_PATH = SOURCES_PATH / "img" / "maps" / "roomfinder"


class OverlayMap(PydanticConfiguration):
    file: str
    floor_index: int
    desc: str
    floor: str
    tumonline: str | None = None


class OverlayProps(PydanticConfiguration):
    parent: str
    box: tuple[tuple[float, float], tuple[float, float], tuple[float, float], tuple[float, float]]


class Overlay(PydanticConfiguration):
    props: OverlayProps
    maps: list[OverlayMap]

    @classmethod
    def load_all(cls) -> dict[str, "Overlay"]:
        """Load all nat.Room's"""
        with open(SOURCES_PATH / "46_overlay-maps.yaml", encoding="utf-8") as file:
            return {_map["props"]["parent"]: cls.model_validate(_map) for _map in yaml.safe_load(file.read())}


class MapKey(typing.NamedTuple):
    building_id: str
    floor: str


class Coordinate(typing.TypedDict):
    lat: float
    lon: float


class CustomMapProps(PydanticConfiguration):
    scale: str
    north: float
    east: float
    south: float
    west: float
    rotation: float
    source: str = "NavigaTUM-Contributors"


class CustomMapItem(PydanticConfiguration):
    file: str
    b_id: str
    desc: str
    floor: str

    def dimensions(self):
        with Image.open(CUSTOM_RF_DIR_PATH / self.file) as img:
            return {"width": img.width, "height": img.height}


class CustomBuildingMap(PydanticConfiguration):
    props: CustomMapProps
    maps: list[CustomMapItem]

    @classmethod
    def load_all_raw(cls) -> list["CustomBuildingMap"]:
        """Load all nat.Room's"""
        with open(SOURCES_PATH / "45_custom-maps.yaml", encoding="utf-8") as file:
            return [cls.model_validate(_map) for _map in yaml.safe_load(file.read())]

    def _as_roomfinder_maps(self) -> dict[MapKey, roomfinder.Map]:
        """Convert to roomfinder.Map"""
        return {
            MapKey(_map.b_id, _map.floor): roomfinder.Map(
                **{
                    "desc": _map.desc,
                    "id": ".".join(_map.file.split(".")[:-1]),
                    "file": _map.file,
                    "source": self.props.source,
                    "scale": self.props.scale,
                    "latlonbox": {
                        "north": self.props.north,
                        "east": self.props.east,
                        "west": self.props.west,
                        "south": self.props.south,
                        "rotation": self.props.rotation,
                    },
                    **_map.dimensions(),
                },
            )
            for _map in self.maps
        }

    @classmethod
    def load_all(cls) -> dict[MapKey, roomfinder.Map]:
        """Load all custom maps as roomfinder.Map's"""
        results: dict[MapKey, roomfinder.Map] = {}
        for _map in cls.load_all_raw():
            results |= _map._as_roomfinder_maps()
        return results
