from setuptools import setup, find_packages

setup(
    name='booking-ms',
    version='0.1.0',
    package_dir={'': 'gen/src/python'},
    install_requires=[
        'betterproto',
    ],
)