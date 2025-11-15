import pytest
from hypothesis import given
from hypothesis import strategies as st
from openspeleo_core.compass_core.models import BaseLocation
from openspeleo_core.compass_core.models import CorrectionFactors
from openspeleo_core.compass_core.models import EastNorthElevation
from openspeleo_core.compass_core.models import FulfordMak
from openspeleo_core.compass_core.models import Location
from openspeleo_core.compass_core.models import Parameters
from openspeleo_core.compass_core.models import ProjectStation
from openspeleo_core.compass_core.models import Shot
from openspeleo_core.compass_core.models import Survey
from openspeleo_core.compass_core.models import SurveyDate
from openspeleo_core.compass_core.models import SurveyFile

# --- Primitive generators ---------------------------------------------------

float_reasonable = st.floats(
    min_value=-1e6, max_value=1e6, allow_nan=False, allow_infinity=False
)
float_elevation = st.floats(min_value=-9999, max_value=9999)
string_small = st.text(min_size=1, max_size=10)

# --- Basic model fuzzing ----------------------------------------------------


@given(float_reasonable, float_reasonable, float_elevation)
def test_location_fuzzy(e, n, u):
    loc = Location(easting=e, northing=n, up=u)
    assert isinstance(loc.easting, float)
    assert -10000 <= loc.up <= 10000


@given(float_reasonable, float_reasonable, float_reasonable)
def test_correction_factors_fuzzy(a, i, l):
    cf = CorrectionFactors(azimuth=a, inclination=i, length=l)
    assert cf.model_dump()["azimuth"] == a


@given(
    st.floats(-20, 20),
    st.builds(CorrectionFactors, float_reasonable, float_reasonable, float_reasonable),
)
def test_parameters_fuzzy(decl, cf):
    params = Parameters(declination=decl, correction_factors=cf)
    assert params.declination == decl


@given(
    string_small,
    string_small,
    st.floats(0, 1000),
    st.floats(0, 360),
    st.floats(-90, 90),
    *(float_elevation for _ in range(4)),
    st.none() | st.text(max_size=10),
    st.none() | st.text(max_size=10),
)
def test_shot_fuzzy(fr, to, length, az, inc, up, down, left, right, flags, comment):
    s = Shot(
        from_=fr,
        to=to,
        length=length,
        azimuth=az,
        inclination=inc,
        up=up,
        down=down,
        left=left,
        right=right,
        flags=flags,
        comment=comment,
    )
    assert s.length >= 0


@given(st.integers(1, 12), st.integers(1, 28), st.integers(1900, 2100))
def test_survey_date_fuzzy(month, day, year):
    d = SurveyDate(month=month, day=day, year=year)
    assert d.as_date.year == year


# --- Higher-order model fuzzing --------------------------------------------


@given(
    string_small,
    string_small,
    st.builds(
        SurveyDate, st.integers(1, 12), st.integers(1, 28), st.integers(1900, 2100)
    ),
    st.text(max_size=30),
    st.text(max_size=30),
    st.builds(
        Parameters,
        st.floats(-30, 30),
        st.builds(
            CorrectionFactors, st.floats(0, 360), st.floats(-90, 90), st.floats(0, 100)
        ),
    ),
    st.lists(
        st.builds(
            Shot,
            from_=string_small,
            to=string_small,
            length=st.floats(0, 1000),
            azimuth=st.floats(0, 360),
            inclination=st.floats(-90, 90),
            up=float_elevation,
            down=float_elevation,
            left=float_elevation,
            right=float_elevation,
            flags=st.none(),
            comment=st.none(),
        ),
        min_size=1,
        max_size=5,
    ),
)
def test_survey_fuzzy(cave_name, name, date, comment, team, params, shots):
    s = Survey(
        cave_name=cave_name,
        name=name,
        date=date,
        comment=comment,
        team=team,
        parameters=params,
        shots=shots,
    )
    assert all(isinstance(shot.length, float) for shot in s.shots)


@given(
    string_small,
    st.builds(Location, float_reasonable, float_reasonable, float_elevation),
)
def test_project_station_fuzzy(name, loc):
    ps = ProjectStation(name=name, location=loc)
    assert isinstance(ps.location.up, float)


@given(
    st.text(min_size=1, max_size=10),
    st.lists(
        st.builds(
            ProjectStation,
            string_small,
            st.builds(Location, float_reasonable, float_reasonable, float_elevation),
        ),
        min_size=1,
        max_size=3,
    ),
    st.lists(
        st.builds(
            Survey,
            string_small,
            string_small,
            st.builds(
                SurveyDate,
                st.integers(1, 12),
                st.integers(1, 28),
                st.integers(1900, 2100),
            ),
            st.text(max_size=10),
            st.text(max_size=10),
            st.builds(
                Parameters,
                st.floats(-30, 30),
                st.builds(
                    CorrectionFactors,
                    st.floats(0, 360),
                    st.floats(-90, 90),
                    st.floats(0, 100),
                ),
            ),
            st.lists(
                st.builds(
                    Shot,
                    from_=string_small,
                    to=string_small,
                    length=st.floats(0, 1000),
                    azimuth=st.floats(0, 360),
                    inclination=st.floats(-90, 90),
                    up=float_elevation,
                    down=float_elevation,
                    left=float_elevation,
                    right=float_elevation,
                    flags=st.none(),
                    comment=st.none(),
                ),
                min_size=1,
                max_size=3,
            ),
        ),
        min_size=1,
        max_size=3,
    ),
)
def test_survey_file_fuzzy(path, stations, surveys):
    sf = SurveyFile(file_path=path, project_stations=stations, surveys=surveys)
    assert isinstance(sf.project_stations[0].location.easting, float)


@given(
    st.builds(EastNorthElevation, float_reasonable, float_reasonable, float_elevation),
    st.integers(1, 60),
    st.floats(-5, 5),
)
def test_base_location_fuzzy(ene, zone, conv):
    bl = BaseLocation(east_north_elevation=ene, zone=zone, convergence_angle=conv)
    assert -5 <= bl.convergence_angle <= 5


@given(
    st.text(min_size=1, max_size=10),
    st.builds(
        BaseLocation,
        st.builds(
            EastNorthElevation, float_reasonable, float_reasonable, float_elevation
        ),
        st.integers(1, 60),
        st.floats(-5, 5),
    ),
    st.text(min_size=1, max_size=10),
    st.none() | st.integers(1, 60),
    st.lists(
        st.builds(
            SurveyFile,
            st.text(min_size=1, max_size=10),
            st.lists(
                st.builds(
                    ProjectStation,
                    string_small,
                    st.builds(
                        Location, float_reasonable, float_reasonable, float_elevation
                    ),
                ),
                min_size=1,
                max_size=2,
            ),
            st.lists(
                st.builds(
                    Survey,
                    string_small,
                    string_small,
                    st.builds(
                        SurveyDate,
                        st.integers(1, 12),
                        st.integers(1, 28),
                        st.integers(1900, 2100),
                    ),
                    st.text(max_size=10),
                    st.text(max_size=10),
                    st.builds(
                        Parameters,
                        st.floats(-30, 30),
                        st.builds(
                            CorrectionFactors,
                            st.floats(0, 360),
                            st.floats(-90, 90),
                            st.floats(0, 100),
                        ),
                    ),
                    st.lists(
                        st.builds(
                            Shot,
                            from_=string_small,
                            to=string_small,
                            length=st.floats(0, 1000),
                            azimuth=st.floats(0, 360),
                            inclination=st.floats(-90, 90),
                            up=float_elevation,
                            down=float_elevation,
                            left=float_elevation,
                            right=float_elevation,
                            flags=st.none(),
                            comment=st.none(),
                        ),
                        min_size=1,
                        max_size=2,
                    ),
                ),
                min_size=1,
                max_size=2,
            ),
        ),
        min_size=1,
        max_size=2,
    ),
)
def test_fulford_mak_fuzzy(path, base_loc, datum, utm, survey_files):
    model = FulfordMak(
        file_path=path,
        base_location=base_loc,
        datum=datum,
        utm_zone=utm,
        survey_files=survey_files,
    )
    js = model.to_json()
    assert isinstance(js, str)
