/*
 Navicat Premium Data Transfer

 Source Server         : localhost
 Source Server Type    : MySQL
 Source Server Version : 80025
 Source Host           : 127.0.0.1:3306
 Source Schema         : COVID-19

 Target Server Type    : MySQL
 Target Server Version : 80025
 File Encoding         : 65001

 Date: 23/04/2022 23:39:32
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for nucleic_test_institution
-- ----------------------------
DROP TABLE IF EXISTS `nucleic_test_institution`;
CREATE TABLE `nucleic_test_institution` (
  `id` varchar(255) COLLATE utf8mb4_general_ci NOT NULL COMMENT '雪花id',
  `institution_name` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '检测机构名称',
  `institution_address` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '检测机构地址',
  `institution_phone` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '检测机构电话',
  `institution_region` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '检测机构所属区域（比如 静安区）',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Table structure for nucleic_test_registe
-- ----------------------------
DROP TABLE IF EXISTS `nucleic_test_registe`;
CREATE TABLE `nucleic_test_registe` (
  `id` varchar(255) COLLATE utf8mb4_general_ci NOT NULL COMMENT '主键 雪花id',
  `nucleic_type` int DEFAULT NULL COMMENT '核酸类型（0-单人单管 1-一户一管 2-混采）',
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户姓名',
  `address` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户居住地址',
  `phone` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户电话',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  `update_time` datetime DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Table structure for nucleic_test_result
-- ----------------------------
DROP TABLE IF EXISTS `nucleic_test_result`;
CREATE TABLE `nucleic_test_result` (
  `id` varchar(255) COLLATE utf8mb4_general_ci NOT NULL COMMENT '主键',
  `result_type` int DEFAULT NULL COMMENT '核酸结果（0-阴性 1-阳性 2-无效）',
  `institution_id` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '核酸机构id 外键',
  `registe_id` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '核酸登记id 外键',
  `create_time` datetime DEFAULT NULL COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Table structure for role
-- ----------------------------
DROP TABLE IF EXISTS `role`;
CREATE TABLE `role` (
  `id` varchar(255) COLLATE utf8mb4_general_ci NOT NULL COMMENT '主键 雪花id',
  `role_code` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '角色编码（唯一）',
  `role_name` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '角色名称',
  `role_status` int DEFAULT '0' COMMENT '角色状态(0-启用 1-停用)',
  `create_time` timestamp NULL DEFAULT NULL COMMENT '创建时间',
  `update_time` timestamp NULL DEFAULT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Table structure for user
-- ----------------------------
DROP TABLE IF EXISTS `user`;
CREATE TABLE `user` (
  `id` varchar(255) COLLATE utf8mb4_general_ci NOT NULL COMMENT '主键',
  `username` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '登陆用户名',
  `password` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '登陆密码',
  `name` varchar(255) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT '用户姓名',
  `sex` int DEFAULT NULL COMMENT '性别 0-女 1-男',
  `brithday` timestamp NULL DEFAULT NULL COMMENT '生日（yyyy-MM-dd）',
  `status` int DEFAULT '0' COMMENT '用户状态 0-启用 1-停用',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

SET FOREIGN_KEY_CHECKS = 1;
