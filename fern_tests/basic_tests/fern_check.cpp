//
// Created by Jeffrey Yu on 4/30/16.
//

#include "gtest/gtest.h"


class FernFixture : public ::testing::Test {

protected:
    virtual void SetUp() {

    }

public:
    FernFixture() : Test() { }
};

TEST_F(FernFixture, add_check) {
    EXPECT_EQ(1, 1);
}
