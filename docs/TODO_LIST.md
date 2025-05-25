<!--
SPDX-FileCopyrightText: 2025 Husamettin ARABACI
SPDX-License-Identifier: MIT
-->

# 📋 TODO List - hexaFn Project

This document tracks the development progress of the hexaFn project organized by milestones and sprints. Each item represents a specific task or feature aligned with the **6F Lifecycle Flow** and **Hexagonal Architecture** principles.

---

## 📊 Progress Overview

| Status | Count | Percentage |
|--------|-------|------------|
| ✅ Completed | 1 | 10% |
| 🔄 In Progress | 3 | 30% |
| 📋 Todo | 6 | 60% |
| **Total** | **10** | **100%** |

---

## 🎯 Milestone 1: Establish Trigger → Run Flow Using DSL

### 🔧 Sprint 1 – Basic Trigger and DSL Foundation

#### ✅ Issue #001: Create Core DSL Parser Infrastructure

- **Module**: `hexafn-run`
- **Priority**: `priority:high`
- **Type**: `type:feature`
- **Status**: ✅ **COMPLETED**
- **Description**: Implement basic DSL parser for hexaFn's internal domain-specific language to enable function definitions and execution
- **6F Phase**: Function
- **Completion**: 100%

#### 🔄 Issue #002: Implement Basic Trigger Detection System

- **Module**: `hexafn-trigger`
- **Priority**: `priority:high`
- **Type**: `type:feature`
- **Status**: 🔄 **IN PROGRESS** (70% complete)
- **Description**: Build foundational trigger system that can detect and evaluate simple event conditions (timer-based, value-based)
- **6F Phase**: Feed → Filter
- **Dependencies**: None
- **Remaining**: Add timer-based trigger validation

#### 📋 Issue #003: Design Trigger → Function Execution Pipeline

- **Module**: `hexafn-core`
- **Priority**: `priority:high`
- **Type**: `type:feature`
- **Status**: 📋 **TODO**
- **Description**: Create the core pipeline that connects trigger events to DSL function execution with proper error handling
- **6F Phase**: All phases integration
- **Dependencies**: #001, #002

### 🗂️ Sprint 2 – Configuration-Based Trigger Management

#### 🔄 Issue #004: Build Configuration File Loader for Triggers

- **Module**: `hexafn-trigger`
- **Priority**: `priority:medium`
- **Type**: `type:feature`
- **Status**: 🔄 **IN PROGRESS** (40% complete)
- **Description**: Enable loading trigger definitions from YAML/TOML configuration files with schema validation
- **6F Phase**: Feed → Format
- **Dependencies**: #002
- **Remaining**: Schema validation and error handling

#### 📋 Issue #005: Implement Trigger Registry with Hot-Reload Support

- **Module**: `hexafn-store`
- **Priority**: `priority:medium`
- **Type**: `type:feature`
- **Status**: 📋 **TODO**
- **Description**: Create in-memory trigger registry that supports dynamic loading/unloading of triggers without restart
- **6F Phase**: Forward
- **Dependencies**: #004

---

## 🌐 Milestone 2: Webhook Integration and Event Broadcasting

### 🌍 Sprint 1 – Webhook Input and Basic Broadcasting

#### 🔄 Issue #006: Create HTTP Webhook Endpoint Infrastructure

- **Module**: `hexafn-bridge`
- **Priority**: `priority:high`
- **Type**: `type:feature`
- **Status**: 🔄 **IN PROGRESS** (25% complete)
- **Description**: Build secure HTTP endpoints to receive webhook events from external systems with authentication
- **6F Phase**: Feed
- **Dependencies**: None
- **Remaining**: Security middleware and request validation

#### 📋 Issue #007: Implement Event Normalization and Validation

- **Module**: `hexafn-bridge`
- **Priority**: `priority:high`
- **Type**: `type:feature`
- **Status**: 📋 **TODO**
- **Description**: Normalize incoming webhook payloads into standard internal event format with JSON schema validation
- **6F Phase**: Filter → Format
- **Dependencies**: #006

### 🔄 Sprint 2 – Robust Pub-Sub System

#### 📋 Issue #008: Build Asynchronous Pub-Sub Event Broadcasting

- **Module**: `hexafn-cast`
- **Priority**: `priority:high`
- **Type**: `type:feature`
- **Status**: 📋 **TODO**
- **Description**: Create high-performance async pub-sub system for internal event broadcasting with topic filtering
- **6F Phase**: Forward
- **Dependencies**: #007

---

## 🔍 Milestone 3: Observability, Logging, and Event Tracing

### 🧵 Sprint 1 – Tracing and Structured Logging Foundation

#### 📋 Issue #009: Implement Distributed Tracing with Correlation IDs

- **Module**: `hexafn-watch`
- **Priority**: `priority:medium`
- **Type**: `type:feature`
- **Status**: 📋 **TODO**
- **Description**: Add trace ID propagation across all 6F phases with OpenTelemetry integration for full event tracking
- **6F Phase**: Feedback
- **Dependencies**: #003, #008

#### 📋 Issue #010: Create Real-time Metrics and Performance Dashboard

- **Module**: `hexafn-watch`
- **Priority**: `priority:low`
- **Type**: `type:enhancement`
- **Status**: 📋 **TODO**
- **Description**: Build Prometheus-compatible metrics endpoint and basic web dashboard for pipeline performance monitoring
- **6F Phase**: Feedback
- **Dependencies**: #009

---

## 🏷️ Issue Labels Reference

### Module Labels

- `module:core` - Core architecture & event flow
- `module:trigger` - HexaTrigger (rule engine)
- `module:run` - HexaRun (function runtime)
- `module:store` - HexaStore (KV storage)
- `module:cast` - HexaCast (pub-sub engine)
- `module:bridge` - HexaBridge (external integrations)
- `module:watch` - HexaWatch (observability)

### Priority Labels

- `priority:high` - Critical for milestone completion
- `priority:medium` - Important but can be delayed
- `priority:low` - Nice to have enhancements

### Type Labels

- `type:feature` - New functionality
- `type:enhancement` - Improvement of existing functionality
- `type:bug` - Defect or malfunction
- `type:refactor` - Code restructuring

---

## 🎯 Next Sprint Planning

### Immediate Focus (Next 2 Weeks)

1. Complete Issue #002 (Trigger Detection System)
2. Complete Issue #004 (Configuration File Loader)
3. Complete Issue #006 (HTTP Webhook Endpoints)

### Dependencies to Watch

- Issue #003 depends on completion of #001 ✅ and #002 🔄
- Issue #005 depends on completion of #004 🔄
- Issue #007 depends on completion of #006 🔄
- Issue #008 depends on completion of #007 📋

---

## 📈 Sprint Velocity Tracking

| Sprint | Planned | Completed | Velocity |
|--------|---------|-----------|----------|
| Sprint 1.1 | 3 issues | 1 issue | 33% |
| Sprint 1.2 | 2 issues | 0 issues | 0% |
| Sprint 2.1 | 2 issues | 0 issues | 0% |
| Sprint 3.1 | 2 issues | 0 issues | 0% |

### 📊 Burn-down Metrics

- **Total Planned**: 10 issues
- **Completed**: 1 issue (10%)
- **In Progress**: 3 issues (30%)
- **Remaining**: 6 issues (60%)
- **Estimated Completion**: 6-8 weeks

---

## 🔄 Weekly Review Process

### Every Monday

- [ ] Review in-progress issues for blockers
- [ ] Update completion percentages
- [ ] Identify new dependencies
- [ ] Adjust sprint priorities

### Every Friday

- [ ] Update sprint velocity metrics
- [ ] Plan next week's focus areas
- [ ] Review milestone progress
- [ ] Update stakeholder communications

---

**Last Updated**: May 25, 2025  
**Next Review**: June 1, 2025  
**Milestone Target**: Q3 2025

---

> 💡 **Note**: This TODO list follows the hexaFn **6F Lifecycle Flow** (Feed → Filter → Format → Function → Forward → Feedback) and **Hexagonal Architecture** principles. All issues are designed to maintain separation of concerns and ensure testable, modular development.