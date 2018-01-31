#include <vector>
#include <array>
#include <numeric>
#include <iostream>
#include <fstream>
#include <iomanip>

std::array<uint8_t,256> run_rounds(const std::vector<uint8_t> inputs,
                                const size_t &hash_length,
                                const size_t &num_rounds)
{
  std::array<uint8_t,256> ring;
  std::iota(ring.begin(), ring.end(), 0);

  size_t skip(0), current(0);
  for (size_t round=0; round<num_rounds; ++round)
    {
      for (auto &input: inputs)
        {
          uint8_t start (current + (input-1));
          uint8_t finish(current);

          const uint8_t stop (input/2);
          for (uint8_t ix=0; ix != stop; ++ix)
            {
              std::swap(ring[start], ring[finish]);
              --start;
              ++finish;
            }
          current += input + skip;
          ++skip;
        }
    }
  return ring;
}

extern "C"{
    uint8_t* wlandry(uint8_t const* ptr, size_t len)
    {
    const size_t hash_length(256);
    std::vector<uint8_t> inputs;
    {
        for (size_t i = 0; i < len; i++) {
            inputs.push_back(ptr[i]);
        }
    }
    for (auto &c: {17, 31, 73, 47, 23})
        { inputs.push_back(c); }
    auto ring(run_rounds(inputs,hash_length,64));

    std::vector<uint8_t> dense_hash(hash_length/16);
    for (size_t ix=0; ix<hash_length; ix+=16)
        { for(size_t jx=0; jx<16; ++jx)
            { dense_hash[ix/16]^=ring[ix+jx]; } }
            return dense_hash.data();
    }

}
