#pragma once

struct nsSize {
  int width, height;
  constexpr nsSize(int aWidth, int aHeight) : width(aWidth), height(aHeight) {}
};
