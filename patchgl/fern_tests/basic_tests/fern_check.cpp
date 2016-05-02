//
// Created by Jeffrey Yu on 4/30/16.
//

#include "gtest/gtest.h"
#include "fern.h"


class FernFixture : public ::testing::Test {

public:
    FernFixture() : Test() { }
};

TEST_F(FernFixture, starts_with_requested_size) {
    fern fern(100);
    int size = fern.size;
    EXPECT_EQ(100, size);
}


TEST_F(FernFixture, starts_with_free_count_equal_to_requested) {
    fern fern(100);
    EXPECT_EQ(100, fern.pool_size());
}


TEST_F(FernFixture, add_child_to_root) {
    fern fern(100);
    const char *hello = "hello";
    int id = fern.add_child(fern.ROOT_ID, hello);
    EXPECT_NE(fern.INVALID_ID, id);
    EXPECT_NE(fern.ROOT_ID, id);
    EXPECT_EQ(99, fern.pool_size());
    EXPECT_EQ(hello, fern.payload_from(id));
    EXPECT_EQ(fern.ROOT_ID, fern.parent_from(id));
    EXPECT_FALSE(fern.is_child(id, fern.ROOT_ID));
    EXPECT_TRUE(fern.is_child(fern.ROOT_ID, id));
}

TEST_F(FernFixture, add_second_child_to_root) {
    fern fern(100);
    const char *hello = "hello";
    const char *goodbye = "goodbye";
    int first_id = fern.add_child(fern.ROOT_ID, hello);
    int id = fern.add_child(fern.ROOT_ID, goodbye);
    EXPECT_NE(fern.INVALID_ID, id);
    EXPECT_NE(fern.ROOT_ID, id);
    EXPECT_NE(first_id, id);
    EXPECT_EQ(98, fern.pool_size());
    EXPECT_EQ(goodbye, fern.payload_from(id));
    EXPECT_EQ(fern.ROOT_ID, fern.parent_from(id));
    EXPECT_FALSE(fern.is_child(id, fern.ROOT_ID));
    EXPECT_FALSE(fern.is_child(first_id, id));
    EXPECT_FALSE(fern.is_child(id, first_id));
    EXPECT_TRUE(fern.is_child(fern.ROOT_ID, id));
    EXPECT_TRUE(fern.are_siblings(id, first_id));
    EXPECT_TRUE(fern.are_siblings(first_id, id));
}

TEST_F(FernFixture, add_grandchild_to_root) {
    fern fern(100);
    const char *hello = "hello";
    const char *goodbye = "goodbye";
    int first_id = fern.add_child(fern.ROOT_ID, hello);
    int id = fern.add_child(first_id, goodbye);
    EXPECT_NE(fern.INVALID_ID, id);
    EXPECT_NE(fern.ROOT_ID, id);
    EXPECT_NE(first_id, id);
    EXPECT_EQ(98, fern.pool_size());
    EXPECT_EQ(goodbye, fern.payload_from(id));
    EXPECT_EQ(first_id, fern.parent_from(id));
    EXPECT_FALSE(fern.are_siblings(id, first_id));
    EXPECT_FALSE(fern.are_siblings(first_id, id));
    EXPECT_FALSE(fern.is_child(id, fern.ROOT_ID));
    EXPECT_FALSE(fern.is_child(fern.ROOT_ID, id));
    EXPECT_FALSE(fern.is_child(id, first_id));
    EXPECT_TRUE(fern.is_child(first_id, id));
}
