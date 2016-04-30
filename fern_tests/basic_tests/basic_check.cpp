//
// Created by Jeffrey Yu on 4/30/16.
//

#include "gtest/gtest.h"

TEST(basic_check, test_eq) {
    EXPECT_EQ(1, 0);
}

TEST(basic_check, test_neq) {
    EXPECT_NE(1, 0);
}
