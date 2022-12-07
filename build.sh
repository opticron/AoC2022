#!/bin/sh
ls *.rs|xargs -n 1 rustc
