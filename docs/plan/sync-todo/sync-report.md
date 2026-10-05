# All Sync Todo Classes (Consolidated)

Generated from all Group files in `docs/plan/sync-todo/` — Classes ordered by name with status and commit ID.

**Status legend:** `[x] Done` = 9-step sync complete AND `# Spec verified:`/`# XSD verified:` stamped in src · `[x]`/`[ ] Deferred` = sync complete (Steps 1–8 green) but the stamp is **deferred to a batch 9b user confirmation** · `[ ] Created` = the class exists in src as an empty stub (dependency placeholder) but implementation has not started · `[ ] Implemented` = the class exists in src with members but the queued 9-step sync is not complete · `[ ] Pending` = the class is not available (not defined in src at all). (Deferred set audited 2026-09-27 against the src stamps.)

## Summary

**1903 classes total**

| Status | Classes | Percent |
| --- | --- | --- |
| [x] Done | 1 | 0.1% |
| [x] Deferred | 0 | 0.0% |
| [x] Retired | 0 | 0.0% |
| [ ] Deferred | 0 | 0.0% |
| [ ] Implemented | 1377 | 72.4% |
| [ ] Created | 522 | 27.4% |
| [ ] Pending | 3 | 0.2% |

| Class Name                                              | Status      | Commit ID                                | Groups           |
| ------------------------------------------------------- | ------------| ---------------------------------------- | ---------------- |
| `ARElement`                                             | [ ] Implemented| N/A                                      | Group1           |
| `ARList`                                                | [ ] Implemented| N/A                                      | Group9           |
| `ARObject`                                              | [x] Done    | 1be5c3b284                               | Group1           |
| `ARPackage`                                             | [ ] Implemented| N/A                                      | Group1           |
| `AUTOSAR`                                               | [ ] Implemented| N/A                                      | Group1           |
| `AbsoluteTolerance`                                     | [ ] Created | N/A                                      | Group31          |
| `AbstractAccessPoint`                                   | [ ] Implemented| N/A                                      | Group22          |
| `AbstractCanCluster`                                    | [ ] Implemented| N/A                                      | Group29          |
| `AbstractCanCommunicationConnector`                     | [ ] Implemented| N/A                                      | Group29          |
| `AbstractCanCommunicationController`                    | [ ] Implemented| N/A                                      | Group29          |
| `AbstractCanCommunicationControllerAttributes`          | [ ] Implemented| N/A                                      | Group29          |
| `AbstractCanPhysicalChannel`                            | [ ] Implemented| N/A                                      | Group29          |
| `AbstractClassTailoring`                                | [ ] Created | N/A                                      | Group36          |
| `AbstractCondition`                                     | [ ] Created | N/A                                      | Group36          |
| `AbstractDoIpLogicAddressProps`                         | [ ] Implemented| N/A                                      | Group7           |
| `AbstractEnumerationValueVariationPoint`                | [ ] Implemented| N/A                                      | Group8           |
| `AbstractEthernetFrame`                                 | [ ] Implemented| N/A                                      | Group6           |
| `AbstractEvent`                                         | [ ] Implemented| N/A                                      | Group28          |
| `AbstractGlobalTimeDomainProps`                         | [ ] Created | N/A                                      | Group34          |
| `AbstractImplementationDataType`                        | [ ] Implemented| N/A                                      | Group1           |
| `AbstractImplementationDataTypeElement`                 | [ ] Implemented| N/A                                      | Group1           |
| `AbstractMultiplicityRestriction`                       | [ ] Created | N/A                                      | Group36          |
| `AbstractNumericalVariationPoint`                       | [ ] Implemented| N/A                                      | Group8           |
| `AbstractProvidedPortPrototype`                         | [ ] Implemented| N/A                                      | Group11          |
| `AbstractRequiredPortPrototype`                         | [ ] Implemented| N/A                                      | Group11          |
| `AbstractRuleBasedValueSpecification`                   | [ ] Implemented| N/A                                      | Group28          |
| `AbstractSecurityEventFilter`                           | [ ] Created | N/A                                      | Group36          |
| `AbstractServiceInstance`                               | [ ] Implemented| N/A                                      | Group32          |
| `AbstractValueRestriction`                              | [ ] Implemented| N/A                                      | Group21          |
| `AbstractVariationRestriction`                          | [ ] Implemented| N/A                                      | Group21          |
| `AccessCount`                                           | [ ] Implemented| N/A                                      | Group22          |
| `AccessCountSet`                                        | [ ] Implemented| N/A                                      | Group22          |
| `AclObjectSet`                                          | [ ] Implemented| N/A                                      | Group22          |
| `AclOperation`                                          | [ ] Implemented| N/A                                      | Group22          |
| `AclPermission`                                         | [ ] Implemented| N/A                                      | Group22          |
| `AclRole`                                               | [ ] Implemented| N/A                                      | Group22          |
| `AclScopeEnum`                                          | [ ] Implemented| N/A                                      | Group22          |
| `AdditionalBindingTimeEnum`                             | [ ] Created | N/A                                      | Group29          |
| `AgeConstraint`                                         | [ ] Implemented| N/A                                      | Group35          |
| `AggregationCondition`                                  | [ ] Created | N/A                                      | Group36          |
| `AggregationTailoring`                                  | [ ] Created | N/A                                      | Group36          |
| `AliasNameAssignment`                                   | [ ] Implemented| N/A                                      | Group23          |
| `AliasNameSet`                                          | [ ] Implemented| N/A                                      | Group23          |
| `AlignEnum`                                             | [ ] Implemented| N/A                                      | Group3           |
| `AlignmentType`                                         | [ ] Implemented| N/A                                      | Group22          |
| `AnalyzedExecutionTime`                                 | [ ] Implemented| N/A                                      | Group23          |
| `AnyInstanceRef`                                        | [ ] Implemented| N/A                                      | Group22          |
| `ApiPrincipleEnum`                                      | [ ] Implemented| N/A                                      | Group10          |
| `AppOsTaskProxyToEcuTaskProxyMapping`                   | [ ] Implemented| N/A                                      | Group18          |
| `ApplicationArrayDataType`                              | [ ] Implemented| N/A                                      | Group28          |
| `ApplicationArrayElement`                               | [ ] Implemented| N/A                                      | Group28          |
| `ApplicationCompositeDataType`                          | [ ] Implemented| N/A                                      | Group2           |
| `ApplicationCompositeDataTypeSubElementRef`             | [ ] Implemented| N/A                                      | Group27          |
| `ApplicationCompositeElementDataPrototype`              | [ ] Implemented| N/A                                      | Group2           |
| `ApplicationCompositeElementInPortInterfaceInstanceRef` | [ ] Implemented| N/A                                      | Group2           |
| `ApplicationDataType`                                   | [ ] Implemented| N/A                                      | Group2           |
| `ApplicationDeferredDataType`                           | [ ] Implemented| N/A                                      | Group1           |
| `ApplicationEndpoint`                                   | [ ] Implemented| N/A                                      | Group32          |
| `ApplicationEntry`                                      | [ ] Implemented| N/A                                      | Group17          |
| `ApplicationError`                                      | [ ] Implemented| N/A                                      | Group27          |
| `ApplicationInterface`                                  | [ ] Implemented| N/A                                      | Group36          |
| `ApplicationPartition`                                  | [ ] Created | N/A                                      | Group30          |
| `ApplicationPartitionToEcuPartitionMapping`             | [ ] Implemented| N/A                                      | Group18          |
| `ApplicationPrimitiveDataType`                          | [ ] Implemented| N/A                                      | Group2           |
| `ApplicationRecordDataType`                             | [ ] Implemented| N/A                                      | Group2           |
| `ApplicationRecordElement`                              | [ ] Implemented| N/A                                      | Group2           |
| `ApplicationRuleBasedValueSpecification`                | [ ] Implemented| N/A                                      | Group28          |
| `ApplicationSwComponentType`                            | [ ] Implemented| N/A                                      | Group27          |
| `ApplicationValueSpecification`                         | [ ] Implemented| N/A                                      | Group28          |
| `ArParameterInImplementationDataInstanceRef`            | [ ] Created | N/A                                      | Group28          |
| `ArVariableInImplementationDataInstanceRef`             | [ ] Implemented| N/A                                      | Group2           |
| `ArbitraryEventTriggering`                              | [ ] Implemented| N/A                                      | Group35          |
| `Area`                                                  | [ ] Implemented| N/A                                      | Group3           |
| `AreaEnumNohref`                                        | [ ] Implemented| N/A                                      | Group3           |
| `AreaEnumShape`                                         | [ ] Implemented| N/A                                      | Group3           |
| `ArgumentDataPrototype`                                 | [ ] Implemented| N/A                                      | Group27          |
| `ArgumentDirectionEnum`                                 | [ ] Implemented| N/A                                      | Group22          |
| `ArrayImplPolicyEnum`                                   | [ ] Implemented| N/A                                      | Group10          |
| `ArraySizeHandlingEnum`                                 | [ ] Implemented| N/A                                      | Group28          |
| `ArraySizeSemanticsEnum`                                | [ ] Implemented| N/A                                      | Group28          |
| `ArrayValueSpecification`                               | [ ] Implemented| N/A                                      | Group3           |
| `AsamRecordLayoutSemantics`                             | [ ] Created | N/A                                      | Group3           |
| `AssemblySwConnector`                                   | [ ] Implemented| N/A                                      | Group2           |
| `AssignFrameId`                                         | [ ] Implemented| N/A                                      | Group31          |
| `AssignFrameIdRange`                                    | [ ] Implemented| N/A                                      | Group31          |
| `AssignNad`                                             | [ ] Implemented| N/A                                      | Group31          |
| `AsynchronousServerCallPoint`                           | [ ] Implemented| N/A                                      | Group2           |
| `AsynchronousServerCallResultPoint`                     | [ ] Implemented| N/A                                      | Group2           |
| `AsynchronousServerCallReturnsEvent`                    | [ ] Implemented| N/A                                      | Group12          |
| `AtomicSwComponentType`                                 | [ ] Implemented| N/A                                      | Group27          |
| `AtpBlueprint`                                          | [ ] Implemented| N/A                                      | Group1           |
| `AtpBlueprintMapping`                                   | [ ] Implemented| N/A                                      | Group1           |
| `AtpBlueprintable`                                      | [ ] Implemented| N/A                                      | Group1           |
| `AtpDefinition`                                         | [ ] Implemented| N/A                                      | Group1           |
| `AtpPrototype`                                          | [ ] Implemented| N/A                                      | Group1           |
| `AtpStructureElement`                                   | [ ] Implemented| N/A                                      | Group1           |
| `AtpType`                                               | [ ] Implemented| N/A                                      | Group1           |
| `AttributeCondition`                                    | [ ] Created | N/A                                      | Group36          |
| `AttributeTailoring`                                    | [ ] Created | N/A                                      | Group36          |
| `AttributeValueVariationPoint`                          | [ ] Implemented| N/A                                      | Group8           |
| `AutoCollectEnum`                                       | [ ] Implemented| N/A                                      | Group1           |
| `AutosarDataPrototype`                                  | [ ] Implemented| N/A                                      | Group28          |
| `AutosarDataType`                                       | [ ] Implemented| N/A                                      | Group1           |
| `AutosarEngineeringObject`                              | [ ] Implemented| N/A                                      | Group22          |
| `AutosarOperationArgumentInstance`                      | [ ] Implemented| N/A                                      | Group8           |
| `AutosarParameterRef`                                   | [ ] Implemented| N/A                                      | Group10          |
| `AutosarVariableInstance`                               | [ ] Implemented| N/A                                      | Group35          |
| `AutosarVariableRef`                                    | [ ] Implemented| N/A                                      | Group10          |
| `BackgroundEvent`                                       | [ ] Implemented| N/A                                      | Group2           |
| `BaseType`                                              | [ ] Implemented| N/A                                      | Group28          |
| `BaseTypeDefinition`                                    | [ ] Implemented| N/A                                      | Group28          |
| `BaseTypeDirectDefinition`                              | [ ] Implemented| N/A                                      | Group28          |
| `Baseline`                                              | [ ] Created | N/A                                      | Group36          |
| `BinaryManifestAddressableObject`                       | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestItem`                                    | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestItemDefinition`                          | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestItemNumericalValue`                      | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestItemPointerValue`                        | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestItemValue`                               | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestMetaDataField`                           | [ ] Created | N/A                                      | Group35          |
| `BinaryManifestProvideResource`                         | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestRequireResource`                         | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestResource`                                | [ ] Created | N/A                                      | Group34          |
| `BinaryManifestResourceDefinition`                      | [ ] Created | N/A                                      | Group34          |
| `BindingTimeEnum`                                       | [ ] Implemented| N/A                                      | Group8           |
| `BlockState`                                            | [ ] Created | N/A                                      | Group36          |
| `BlueprintFormula`                                      | [ ] Implemented| N/A                                      | Group8           |
| `BlueprintGenerator`                                    | [ ] Implemented| N/A                                      | Group8           |
| `BlueprintMapping`                                      | [ ] Implemented| N/A                                      | Group8           |
| `BlueprintMappingSet`                                   | [ ] Implemented| N/A                                      | Group1           |
| `BlueprintPolicy`                                       | [ ] Implemented| N/A                                      | Group1           |
| `BooleanValueVariationPoint`                            | [ ] Implemented| N/A                                      | Group8           |
| `Br`                                                    | [ ] Implemented| N/A                                      | Group3           |
| `BswApiOptions`                                         | [ ] Implemented| N/A                                      | Group13          |
| `BswAsynchronousServerCallPoint`                        | [ ] Implemented| N/A                                      | Group22          |
| `BswAsynchronousServerCallResultPoint`                  | [ ] Implemented| N/A                                      | Group22          |
| `BswAsynchronousServerCallReturnsEvent`                 | [ ] Implemented| N/A                                      | Group13          |
| `BswBackgroundEvent`                                    | [ ] Implemented| N/A                                      | Group22          |
| `BswCallType`                                           | [ ] Implemented| N/A                                      | Group22          |
| `BswCalledEntity`                                       | [ ] Implemented| N/A                                      | Group22          |
| `BswClientPolicy`                                       | [ ] Implemented| N/A                                      | Group4           |
| `BswCompositionTiming`                                  | [ ] Created | N/A                                      | Group35          |
| `BswDataReceivedEvent`                                  | [ ] Implemented| N/A                                      | Group13          |
| `BswDataReceptionPolicy`                                | [ ] Implemented| N/A                                      | Group13          |
| `BswDataSendPolicy`                                     | [ ] Implemented| N/A                                      | Group4           |
| `BswDirectCallPoint`                                    | [ ] Implemented| N/A                                      | Group13          |
| `BswDistinguishedPartition`                             | [ ] Implemented| N/A                                      | Group22          |
| `BswEntryKindEnum`                                      | [ ] Implemented| N/A                                      | Group22          |
| `BswEntryRelationship`                                  | [ ] Implemented| N/A                                      | Group13          |
| `BswEntryRelationshipEnum`                              | [ ] Implemented| N/A                                      | Group13          |
| `BswEntryRelationshipSet`                               | [ ] Implemented| N/A                                      | Group13          |
| `BswEvent`                                              | [ ] Implemented| N/A                                      | Group22          |
| `BswExclusiveAreaPolicy`                                | [ ] Implemented| N/A                                      | Group22          |
| `BswExecutionContext`                                   | [ ] Implemented| N/A                                      | Group22          |
| `BswExternalTriggerOccurredEvent`                       | [ ] Implemented| N/A                                      | Group22          |
| `BswImplementation`                                     | [ ] Implemented| N/A                                      | Group22          |
| `BswInternalBehavior`                                   | [ ] Implemented| N/A                                      | Group4           |
| `BswInternalTriggerOccurredEvent`                       | [ ] Implemented| N/A                                      | Group13          |
| `BswInternalTriggeringPoint`                            | [ ] Implemented| N/A                                      | Group13          |
| `BswInternalTriggeringPointPolicy`                      | [ ] Implemented| N/A                                      | Group4           |
| `BswInterruptCategory`                                  | [ ] Implemented| N/A                                      | Group22          |
| `BswInterruptEntity`                                    | [ ] Implemented| N/A                                      | Group13          |
| `BswInterruptEvent`                                     | [ ] Implemented| N/A                                      | Group22          |
| `BswMgrNeeds`                                           | [ ] Implemented| N/A                                      | Group4           |
| `BswModeManagerErrorEvent`                              | [ ] Implemented| N/A                                      | Group13          |
| `BswModeReceiverPolicy`                                 | [ ] Implemented| N/A                                      | Group22          |
| `BswModeSenderPolicy`                                   | [ ] Implemented| N/A                                      | Group22          |
| `BswModeSwitchAckRequest`                               | [ ] Implemented| N/A                                      | Group13          |
| `BswModeSwitchEvent`                                    | [ ] Implemented| N/A                                      | Group22          |
| `BswModeSwitchedAckEvent`                               | [ ] Implemented| N/A                                      | Group13          |
| `BswModuleCallPoint`                                    | [ ] Implemented| N/A                                      | Group13          |
| `BswModuleClientServerEntry`                            | [ ] Implemented| N/A                                      | Group13          |
| `BswModuleDependency`                                   | [ ] Implemented| N/A                                      | Group13          |
| `BswModuleDescription`                                  | [ ] Implemented| N/A                                      | Group22          |
| `BswModuleEntity`                                       | [ ] Implemented| N/A                                      | Group22          |
| `BswModuleEntry`                                        | [ ] Implemented| N/A                                      | Group22          |
| `BswModuleTiming`                                       | [ ] Created | N/A                                      | Group35          |
| `BswOperationInvokedEvent`                              | [ ] Implemented| N/A                                      | Group22          |
| `BswOsTaskExecutionEvent`                               | [ ] Implemented| N/A                                      | Group22          |
| `BswParameterPolicy`                                    | [ ] Implemented| N/A                                      | Group4           |
| `BswPerInstanceMemoryPolicy`                            | [ ] Implemented| N/A                                      | Group4           |
| `BswQueuedDataReceptionPolicy`                          | [ ] Implemented| N/A                                      | Group13          |
| `BswReleasedTriggerPolicy`                              | [ ] Implemented| N/A                                      | Group4           |
| `BswSchedulableEntity`                                  | [ ] Implemented| N/A                                      | Group22          |
| `BswScheduleEvent`                                      | [ ] Implemented| N/A                                      | Group22          |
| `BswSchedulerNamePrefix`                                | [ ] Implemented| N/A                                      | Group22          |
| `BswServiceDependency`                                  | [ ] Implemented| N/A                                      | Group23          |
| `BswServiceDependencyIdent`                             | [ ] Implemented| N/A                                      | Group26          |
| `BswSynchronousServerCallPoint`                         | [ ] Implemented| N/A                                      | Group13          |
| `BswTimingEvent`                                        | [ ] Implemented| N/A                                      | Group13          |
| `BswTriggerDirectImplementation`                        | [ ] Implemented| N/A                                      | Group22          |
| `BswVariableAccess`                                     | [ ] Implemented| N/A                                      | Group22          |
| `BufferProperties`                                      | [ ] Implemented| N/A                                      | Group28          |
| `BuildAction`                                           | [ ] Implemented| N/A                                      | Group1           |
| `BuildActionEntity`                                     | [ ] Implemented| N/A                                      | Group1           |
| `BuildActionEnvironment`                                | [ ] Implemented| N/A                                      | Group1           |
| `BuildActionInvocator`                                  | [ ] Implemented| N/A                                      | Group1           |
| `BuildActionIoElement`                                  | [ ] Implemented| N/A                                      | Group1           |
| `BuildActionManifest`                                   | [ ] Implemented| N/A                                      | Group1           |
| `BuildEngineeringObject`                                | [ ] Implemented| N/A                                      | Group1           |
| `BulkNvDataDescriptor`                                  | [ ] Implemented| N/A                                      | Group10          |
| `BurstPatternEventTriggering`                           | [ ] Implemented| N/A                                      | Group35          |
| `BusMirrorCanIdRangeMapping`                            | [ ] Created | N/A                                      | Group34          |
| `BusMirrorCanIdToCanIdMapping`                          | [ ] Created | N/A                                      | Group34          |
| `BusMirrorChannel`                                      | [ ] Created | N/A                                      | Group33          |
| `BusMirrorChannelMapping`                               | [ ] Created | N/A                                      | Group33          |
| `BusMirrorChannelMappingCan`                            | [ ] Created | N/A                                      | Group34          |
| `BusMirrorChannelMappingFlexray`                        | [ ] Created | N/A                                      | Group34          |
| `BusMirrorChannelMappingIp`                             | [ ] Created | N/A                                      | Group34          |
| `BusMirrorChannelMappingUserDefined`                    | [ ] Created | N/A                                      | Group34          |
| `BusMirrorLinPidToCanIdMapping`                         | [ ] Created | N/A                                      | Group34          |
| `BusspecificNmEcu`                                      | [ ] Implemented| N/A                                      | Group33          |
| `ByteOrderEnum`                                         | [ ] Implemented| N/A                                      | Group28          |
| `CIdentifier`                                           | [ ] Implemented| N/A                                      | Group21          |
| `CSTransformerErrorReactionEnum`                        | [ ] Implemented| N/A                                      | Group34          |
| `CalibrationParameterValue`                             | [ ] Created | N/A                                      | Group28          |
| `CalibrationParameterValueSet`                          | [ ] Created | N/A                                      | Group28          |
| `CalprmAxisCategoryEnum`                                | [ ] Implemented| N/A                                      | Group28          |
| `CanAddressingModeType`                                 | [ ] Implemented| N/A                                      | Group32          |
| `CanCluster`                                            | [ ] Implemented| N/A                                      | Group29          |
| `CanClusterBusOffRecovery`                              | [ ] Implemented| N/A                                      | Group17          |
| `CanCommunicationConnector`                             | [ ] Implemented| N/A                                      | Group17          |
| `CanCommunicationController`                            | [ ] Implemented| N/A                                      | Group29          |
| `CanControllerConfiguration`                            | [ ] Implemented| N/A                                      | Group17          |
| `CanControllerConfigurationRequirements`                | [ ] Implemented| N/A                                      | Group17          |
| `CanControllerFdConfiguration`                          | [ ] Implemented| N/A                                      | Group29          |
| `CanControllerFdConfigurationRequirements`              | [ ] Implemented| N/A                                      | Group17          |
| `CanControllerXlConfiguration`                          | [ ] Implemented| N/A                                      | Group29          |
| `CanControllerXlConfigurationRequirements`              | [ ] Implemented| N/A                                      | Group29          |
| `CanFrame`                                              | [ ] Implemented| N/A                                      | Group32          |
| `CanFrameRxBehaviorEnum`                                | [ ] Implemented| N/A                                      | Group32          |
| `CanFrameTriggering`                                    | [ ] Implemented| N/A                                      | Group32          |
| `CanFrameTxBehaviorEnum`                                | [ ] Implemented| N/A                                      | Group32          |
| `CanGlobalTimeDomainProps`                              | [ ] Created | N/A                                      | Group34          |
| `CanNmCluster`                                          | [ ] Implemented| N/A                                      | Group18          |
| `CanNmClusterCoupling`                                  | [ ] Implemented| N/A                                      | Group18          |
| `CanNmEcu`                                              | [ ] Implemented| N/A                                      | Group33          |
| `CanNmNode`                                             | [ ] Implemented| N/A                                      | Group18          |
| `CanPhysicalChannel`                                    | [ ] Implemented| N/A                                      | Group29          |
| `CanTpAddress`                                          | [ ] Implemented| N/A                                      | Group33          |
| `CanTpAddressingFormatType`                             | [ ] Implemented| N/A                                      | Group33          |
| `CanTpChannel`                                          | [ ] Implemented| N/A                                      | Group33          |
| `CanTpConfig`                                           | [ ] Implemented| N/A                                      | Group33          |
| `CanTpConnection`                                       | [ ] Implemented| N/A                                      | Group33          |
| `CanTpEcu`                                              | [ ] Implemented| N/A                                      | Group33          |
| `CanTpNode`                                             | [ ] Implemented| N/A                                      | Group33          |
| `CategoryString`                                        | [ ] Implemented| N/A                                      | Group21          |
| `Chapter`                                               | [ ] Implemented| N/A                                      | Group22          |
| `ChapterContent`                                        | [ ] Implemented| N/A                                      | Group9           |
| `ChapterEnumBreak`                                      | [ ] Implemented| N/A                                      | Group3           |
| `ChapterModel`                                          | [ ] Implemented| N/A                                      | Group9           |
| `ChapterOrMsrQuery`                                     | [ ] Implemented| N/A                                      | Group22          |
| `ClassContentConditional`                               | [ ] Created | N/A                                      | Group36          |
| `ClassTailoring`                                        | [ ] Created | N/A                                      | Group36          |
| `ClientComSpec`                                         | [ ] Implemented| N/A                                      | Group27          |
| `ClientIdDefinition`                                    | [ ] Implemented| N/A                                      | Group5           |
| `ClientIdDefinitionSet`                                 | [ ] Implemented| N/A                                      | Group5           |
| `ClientIdRange`                                         | [ ] Implemented| N/A                                      | Group5           |
| `ClientServerAnnotation`                                | [ ] Implemented| N/A                                      | Group27          |
| `ClientServerApplicationErrorMapping`                   | [ ] Implemented| N/A                                      | Group11          |
| `ClientServerInterface`                                 | [ ] Implemented| N/A                                      | Group27          |
| `ClientServerInterfaceMapping`                          | [ ] Implemented| N/A                                      | Group11          |
| `ClientServerOperation`                                 | [ ] Implemented| N/A                                      | Group27          |
| `ClientServerOperationBlueprintMapping`                 | [ ] Created | N/A                                      | Group36          |
| `ClientServerOperationComProps`                         | [ ] Created | N/A                                      | Group34          |
| `ClientServerOperationMapping`                          | [ ] Implemented| N/A                                      | Group11          |
| `ClientServerToSignalMapping`                           | [ ] Created | N/A                                      | Group31          |
| `Code`                                                  | [ ] Implemented| N/A                                      | Group1           |
| `CollectableElement`                                    | [ ] Implemented| N/A                                      | Group1           |
| `Collection`                                            | [ ] Implemented| N/A                                      | Group1           |
| `Colspec`                                               | [ ] Implemented| N/A                                      | Group3           |
| `ComManagementMapping`                                  | [ ] Implemented| N/A                                      | Group5           |
| `ComMgrUserNeeds`                                       | [ ] Implemented| N/A                                      | Group23          |
| `CommConnectorPort`                                     | [ ] Implemented| N/A                                      | Group31          |
| `CommonSignalPath`                                      | [ ] Created | N/A                                      | Group31          |
| `CommunicationBufferLocking`                            | [ ] Implemented| N/A                                      | Group2           |
| `CommunicationCluster`                                  | [ ] Implemented| N/A                                      | Group24          |
| `CommunicationConnector`                                | [ ] Implemented| N/A                                      | Group29          |
| `CommunicationController`                               | [ ] Implemented| N/A                                      | Group27          |
| `CommunicationControllerMapping`                        | [ ] Implemented| N/A                                      | Group7           |
| `CommunicationCycle`                                    | [ ] Implemented| N/A                                      | Group5           |
| `CommunicationDirectionType`                            | [ ] Implemented| N/A                                      | Group15          |
| `Compiler`                                              | [ ] Implemented| N/A                                      | Group1           |
| `ComplexDeviceDriverSwComponentType`                    | [ ] Implemented| N/A                                      | Group29          |
| `ComponentClustering`                                   | [ ] Created | N/A                                      | Group30          |
| `ComponentInCompositionInstanceRef`                     | [ ] Implemented| N/A                                      | Group7           |
| `ComponentInSystemInstanceRef`                          | [ ] Implemented| N/A                                      | Group7           |
| `ComponentSeparation`                                   | [ ] Created | N/A                                      | Group30          |
| `CompositeNetworkRepresentation`                        | [ ] Implemented| N/A                                      | Group10          |
| `CompositeRuleBasedValueArgument`                       | [ ] Implemented| N/A                                      | Group3           |
| `CompositeRuleBasedValueSpecification`                  | [ ] Implemented| N/A                                      | Group3           |
| `CompositeValueSpecification`                           | [ ] Implemented| N/A                                      | Group3           |
| `CompositionSwComponentType`                            | [ ] Implemented| N/A                                      | Group2           |
| `Compu`                                                 | [ ] Implemented| N/A                                      | Group3           |
| `CompuConst`                                            | [ ] Implemented| N/A                                      | Group3           |
| `CompuConstContent`                                     | [ ] Implemented| N/A                                      | Group3           |
| `CompuConstFormulaContent`                              | [ ] Implemented| N/A                                      | Group3           |
| `CompuConstNumericContent`                              | [ ] Implemented| N/A                                      | Group3           |
| `CompuConstTextContent`                                 | [ ] Implemented| N/A                                      | Group3           |
| `CompuContent`                                          | [ ] Implemented| N/A                                      | Group3           |
| `CompuGenericMath`                                      | [ ] Implemented| N/A                                      | Group3           |
| `CompuMethod`                                           | [ ] Implemented| N/A                                      | Group3           |
| `CompuNominatorDenominator`                             | [ ] Implemented| N/A                                      | Group3           |
| `CompuRationalCoeffs`                                   | [ ] Implemented| N/A                                      | Group3           |
| `CompuScale`                                            | [ ] Implemented| N/A                                      | Group3           |
| `CompuScaleConstantContents`                            | [ ] Implemented| N/A                                      | Group3           |
| `CompuScaleContents`                                    | [ ] Implemented| N/A                                      | Group3           |
| `CompuScaleRationalFormula`                             | [ ] Implemented| N/A                                      | Group3           |
| `CompuScales`                                           | [ ] Implemented| N/A                                      | Group3           |
| `ConcreteClassTailoring`                                | [ ] Created | N/A                                      | Group36          |
| `ConcretePatternEventTriggering`                        | [ ] Implemented| N/A                                      | Group35          |
| `ConditionByFormula`                                    | [ ] Implemented| N/A                                      | Group8           |
| `ConditionalChangeNad`                                  | [ ] Implemented| N/A                                      | Group32          |
| `ConfidenceInterval`                                    | [ ] Implemented| N/A                                      | Group35          |
| `ConfigReferenceValue`                                  | [ ] Implemented| N/A                                      | Group19          |
| `ConsistencyNeeds`                                      | [ ] Implemented| N/A                                      | Group28          |
| `ConstantReference`                                     | [ ] Implemented| N/A                                      | Group9           |
| `ConstantSpecification`                                 | [ ] Implemented| N/A                                      | Group9           |
| `ConstantSpecificationMapping`                          | [ ] Implemented| N/A                                      | Group28          |
| `ConstantSpecificationMappingSet`                       | [ ] Implemented| N/A                                      | Group1           |
| `ConstraintTailoring`                                   | [ ] Created | N/A                                      | Group36          |
| `ConsumedEventGroup`                                    | [ ] Implemented| N/A                                      | Group32          |
| `ConsumedProvidedServiceInstanceGroup`                  | [ ] Implemented| N/A                                      | Group5           |
| `ConsumedServiceInstance`                               | [ ] Implemented| N/A                                      | Group32          |
| `ContainedIPduCollectionSemanticsEnum`                  | [ ] Implemented| N/A                                      | Group5           |
| `ContainedIPduProps`                                    | [ ] Implemented| N/A                                      | Group5           |
| `ContainerIPdu`                                         | [ ] Created | N/A                                      | Group31          |
| `ContainerIPduHeaderTypeEnum`                           | [ ] Created | N/A                                      | Group31          |
| `ContainerIPduTriggerEnum`                              | [ ] Created | N/A                                      | Group31          |
| `CouplingElement`                                       | [ ] Created | N/A                                      | Group30          |
| `CouplingElementAbstractDetails`                        | [ ] Created | N/A                                      | Group30          |
| `CouplingElementEnum`                                   | [ ] Created | N/A                                      | Group30          |
| `CouplingElementSwitchDetails`                          | [ ] Created | N/A                                      | Group30          |
| `CouplingPort`                                          | [ ] Implemented| N/A                                      | Group30          |
| `CouplingPortAbstractShaper`                            | [ ] Implemented| N/A                                      | Group16          |
| `CouplingPortAsynchronousTrafficShaper`                 | [ ] Pending | N/A                                      | Group16          |
| `CouplingPortConnection`                                | [ ] Implemented| N/A                                      | Group30          |
| `CouplingPortCreditBasedShaper`                         | [ ] Pending | N/A                                      | Group16          |
| `CouplingPortDetails`                                   | [ ] Implemented| N/A                                      | Group30          |
| `CouplingPortFifo`                                      | [ ] Implemented| N/A                                      | Group30          |
| `CouplingPortRatePolicy`                                | [ ] Implemented| N/A                                      | Group30          |
| `CouplingPortRatePolicyActionEnum`                      | [ ] Implemented| N/A                                      | Group30          |
| `CouplingPortScheduler`                                 | [ ] Implemented| N/A                                      | Group6           |
| `CouplingPortShaper`                                    | [ ] Created | N/A                                      | Group30          |
| `CouplingPortStructuralElement`                         | [ ] Implemented| N/A                                      | Group6           |
| `CouplingPortTrafficClassAssignment`                    | [ ] Implemented| N/A                                      | Group30          |
| `CpSoftwareCluster`                                     | [ ] Implemented| N/A                                      | Group5           |
| `CpSoftwareClusterBinaryManifestDescriptor`             | [ ] Created | N/A                                      | Group34          |
| `CpSoftwareClusterCommunicationResource`                | [ ] Created | N/A                                      | Group34          |
| `CpSoftwareClusterCommunicationResourceProps`           | [ ] Created | N/A                                      | Group34          |
| `CpSoftwareClusterMappingSet`                           | [ ] Created | N/A                                      | Group31          |
| `CpSoftwareClusterResource`                             | [ ] Created | N/A                                      | Group26          |
| `CpSoftwareClusterResourcePool`                         | [ ] Created | N/A                                      | Group34          |
| `CpSoftwareClusterResourceToApplicationPartitionMapping` | [ ] Created | N/A                                      | Group31          |
| `CpSoftwareClusterServiceResource`                      | [ ] Created | N/A                                      | Group34          |
| `CpSoftwareClusterToApplicationPartitionMapping`        | [ ] Created | N/A                                      | Group31          |
| `CpSoftwareClusterToEcuInstanceMapping`                 | [ ] Created | N/A                                      | Group31          |
| `CpSoftwareClusterToResourceMapping`                    | [ ] Created | N/A                                      | Group34          |
| `CpSwClusterResourceToDiagDataElemMapping`              | [ ] Created | N/A                                      | Group26          |
| `CpSwClusterResourceToDiagFunctionIdMapping`            | [ ] Created | N/A                                      | Group26          |
| `CpSwClusterToDiagEventMapping`                         | [ ] Created | N/A                                      | Group26          |
| `CpSwClusterToDiagRoutineSubfunctionMapping`            | [ ] Created | N/A                                      | Group26          |
| `CryptoCertificateAlgorithmFamilyEnum`                  | [ ] Implemented| N/A                                      | Group6           |
| `CryptoCertificateFormatEnum`                           | [ ] Implemented| N/A                                      | Group6           |
| `CryptoEllipticCurveProps`                              | [ ] Implemented| N/A                                      | Group6           |
| `CryptoKeyManagementNeeds`                              | [ ] Implemented| N/A                                      | Group4           |
| `CryptoKeySlot`                                         | [ ] Implemented| N/A                                      | Group7           |
| `CryptoKeySlotAllowedModification`                      | [ ] Implemented| N/A                                      | Group20          |
| `CryptoKeySlotContentAllowedUsage`                      | [ ] Implemented| N/A                                      | Group20          |
| `CryptoKeySlotTypeEnum`                                 | [ ] Implemented| N/A                                      | Group20          |
| `CryptoObjectTypeEnum`                                  | [ ] Implemented| N/A                                      | Group20          |
| `CryptoServiceCertificate`                              | [ ] Implemented| N/A                                      | Group6           |
| `CryptoServiceJobNeeds`                                 | [ ] Implemented| N/A                                      | Group4           |
| `CryptoServiceKey`                                      | [ ] Created | N/A                                      | Group31          |
| `CryptoServiceKeyGenerationEnum`                        | [ ] Created | N/A                                      | Group31          |
| `CryptoServiceMapping`                                  | [ ] Implemented| N/A                                      | Group6           |
| `CryptoServiceNeeds`                                    | [ ] Implemented| N/A                                      | Group14          |
| `CryptoServicePrimitive`                                | [ ] Implemented| N/A                                      | Group6           |
| `CryptoServiceQueue`                                    | [ ] Created | N/A                                      | Group31          |
| `CryptoSignatureScheme`                                 | [ ] Implemented| N/A                                      | Group6           |
| `CseCodeType`                                           | [ ] Implemented| N/A                                      | Group21          |
| `CycleCounter`                                          | [ ] Implemented| N/A                                      | Group5           |
| `CycleRepetition`                                       | [ ] Implemented| N/A                                      | Group5           |
| `CycleRepetitionType`                                   | [ ] Implemented| N/A                                      | Group5           |
| `CyclicTiming`                                          | [ ] Implemented| N/A                                      | Group15          |
| `DataComProps`                                          | [ ] Created | N/A                                      | Group34          |
| `DataConsistencyPolicyEnum`                             | [ ] Created | N/A                                      | Group34          |
| `DataConstr`                                            | [ ] Implemented| N/A                                      | Group3           |
| `DataConstrRule`                                        | [ ] Implemented| N/A                                      | Group3           |
| `DataDumpEntry`                                         | [ ] Implemented| N/A                                      | Group32          |
| `DataExchangePoint`                                     | [ ] Created | N/A                                      | Group36          |
| `DataExchangePointKind`                                 | [ ] Created | N/A                                      | Group36          |
| `DataFilter`                                            | [ ] Implemented| N/A                                      | Group9           |
| `DataFilterTypeEnum`                                    | [ ] Implemented| N/A                                      | Group9           |
| `DataFormatElementReference`                            | [ ] Created | N/A                                      | Group36          |
| `DataFormatElementScope`                                | [ ] Created | N/A                                      | Group36          |
| `DataIdModeEnum`                                        | [ ] Implemented| N/A                                      | Group34          |
| `DataInterface`                                         | [ ] Implemented| N/A                                      | Group1           |
| `DataLimitKindEnum`                                     | [ ] Implemented| N/A                                      | Group27          |
| `DataLinkLayerRule`                                     | [ ] Implemented| N/A                                      | Group20          |
| `DataMapping`                                           | [ ] Implemented| N/A                                      | Group17          |
| `DataPrototype`                                         | [ ] Implemented| N/A                                      | Group1           |
| `DataPrototypeGroup`                                    | [ ] Implemented| N/A                                      | Group28          |
| `DataPrototypeInClientServerInterfaceInstanceRef`       | [ ] Implemented| N/A                                      | Group34          |
| `DataPrototypeInPortInterfaceRef`                       | [ ] Implemented| N/A                                      | Group34          |
| `DataPrototypeInSenderReceiverInterfaceInstanceRef`     | [ ] Implemented| N/A                                      | Group34          |
| `DataPrototypeMapping`                                  | [ ] Implemented| N/A                                      | Group27          |
| `DataPrototypeReference`                                | [ ] Implemented| N/A                                      | Group34          |
| `DataPrototypeTransformationProps`                      | [ ] Implemented| N/A                                      | Group6           |
| `DataReceiveErrorEvent`                                 | [ ] Implemented| N/A                                      | Group12          |
| `DataReceivedEvent`                                     | [ ] Implemented| N/A                                      | Group12          |
| `DataSendCompletedEvent`                                | [ ] Implemented| N/A                                      | Group12          |
| `DataTransformation`                                    | [ ] Implemented| N/A                                      | Group27          |
| `DataTransformationErrorHandlingEnum`                   | [ ] Implemented| N/A                                      | Group2           |
| `DataTransformationKindEnum`                            | [ ] Implemented| N/A                                      | Group27          |
| `DataTransformationSet`                                 | [ ] Implemented| N/A                                      | Group6           |
| `DataTransformationStatusForwardingEnum`                | [ ] Implemented| N/A                                      | Group2           |
| `DataTypeMap`                                           | [ ] Implemented| N/A                                      | Group10          |
| `DataTypeMappingSet`                                    | [ ] Implemented| N/A                                      | Group2           |
| `DataTypePolicyEnum`                                    | [ ] Implemented| N/A                                      | Group31          |
| `DataWriteCompletedEvent`                               | [ ] Implemented| N/A                                      | Group12          |
| `DateTime`                                              | [ ] Implemented| N/A                                      | Group21          |
| `DcmIPdu`                                               | [ ] Implemented| N/A                                      | Group31          |
| `DdsCpConfig`                                           | [ ] Created | N/A                                      | Group32          |
| `DdsCpConsumedServiceInstance`                          | [ ] Created | N/A                                      | Group32          |
| `DdsCpDomain`                                           | [ ] Created | N/A                                      | Group32          |
| `DdsCpISignalToDdsTopicMapping`                         | [ ] Created | N/A                                      | Group31          |
| `DdsCpPartition`                                        | [ ] Created | N/A                                      | Group32          |
| `DdsCpProvidedServiceInstance`                          | [ ] Created | N/A                                      | Group32          |
| `DdsCpQosProfile`                                       | [ ] Created | N/A                                      | Group32          |
| `DdsCpServiceInstance`                                  | [ ] Created | N/A                                      | Group32          |
| `DdsCpServiceInstanceEvent`                             | [ ] Created | N/A                                      | Group32          |
| `DdsCpServiceInstanceOperation`                         | [ ] Created | N/A                                      | Group32          |
| `DdsCpTopic`                                            | [ ] Created | N/A                                      | Group32          |
| `DdsDeadline`                                           | [ ] Created | N/A                                      | Group32          |
| `DdsDestinationOrder`                                   | [ ] Created | N/A                                      | Group32          |
| `DdsDestinationOrderKindEnum`                           | [ ] Created | N/A                                      | Group32          |
| `DdsDurability`                                         | [ ] Created | N/A                                      | Group32          |
| `DdsDurabilityKindEnum`                                 | [ ] Created | N/A                                      | Group32          |
| `DdsDurabilityService`                                  | [ ] Created | N/A                                      | Group32          |
| `DdsDurabilityServiceHistoryKindEnum`                   | [ ] Created | N/A                                      | Group32          |
| `DdsHistory`                                            | [ ] Created | N/A                                      | Group32          |
| `DdsHistoryKindEnum`                                    | [ ] Created | N/A                                      | Group32          |
| `DdsLatencyBudget`                                      | [ ] Created | N/A                                      | Group32          |
| `DdsLifespan`                                           | [ ] Created | N/A                                      | Group32          |
| `DdsLiveliness`                                         | [ ] Created | N/A                                      | Group32          |
| `DdsLivenessKindEnum`                                   | [ ] Created | N/A                                      | Group32          |
| `DdsOwnership`                                          | [ ] Created | N/A                                      | Group32          |
| `DdsOwnershipKindEnum`                                  | [ ] Created | N/A                                      | Group32          |
| `DdsOwnershipStrength`                                  | [ ] Created | N/A                                      | Group32          |
| `DdsReliability`                                        | [ ] Created | N/A                                      | Group32          |
| `DdsReliabilityKindEnum`                                | [ ] Created | N/A                                      | Group32          |
| `DdsResourceLimits`                                     | [ ] Created | N/A                                      | Group32          |
| `DdsTopicData`                                          | [ ] Created | N/A                                      | Group32          |
| `DdsTransportPriority`                                  | [ ] Created | N/A                                      | Group32          |
| `DefItem`                                               | [ ] Implemented| N/A                                      | Group21          |
| `DefList`                                               | [ ] Implemented| N/A                                      | Group21          |
| `DefaultValueApplicationStrategyEnum`                   | [ ] Created | N/A                                      | Group36          |
| `DefaultValueElement`                                   | [ ] Implemented| N/A                                      | Group17          |
| `DelegatedPortAnnotation`                               | [ ] Implemented| N/A                                      | Group27          |
| `DelegationSwConnector`                                 | [ ] Implemented| N/A                                      | Group2           |
| `DependencyOnArtifact`                                  | [ ] Implemented| N/A                                      | Group1           |
| `DependencyUsageEnum`                                   | [ ] Implemented| N/A                                      | Group10          |
| `DevelopmentError`                                      | [ ] Implemented| N/A                                      | Group23          |
| `DhcpServerConfiguration`                               | [ ] Implemented| N/A                                      | Group30          |
| `Dhcpv6Props`                                           | [ ] Created | N/A                                      | Group30          |
| `DiagEventDebounceAlgorithm`                            | [ ] Implemented| N/A                                      | Group4           |
| `DiagEventDebounceCounterBased`                         | [ ] Implemented| N/A                                      | Group14          |
| `DiagEventDebounceMonitorInternal`                      | [ ] Implemented| N/A                                      | Group4           |
| `DiagEventDebounceTimeBased`                            | [ ] Implemented| N/A                                      | Group23          |
| `DiagPduType`                                           | [ ] Created | N/A                                      | Group31          |
| `DiagRequirementIdString`                               | [ ] Implemented| N/A                                      | Group21          |
| `DiagnosticAbstractAliasEvent`                          | [ ] Created | N/A                                      | Group25          |
| `DiagnosticAbstractDataIdentifier`                      | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticAbstractParameter`                           | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticAccessPermission`                            | [ ] Implemented| N/A                                      | Group7           |
| `DiagnosticAging`                                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticAudienceEnum`                                | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticAuthRole`                                    | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticAuthRoleProxy`                               | [ ] Implemented| N/A                                      | Group7           |
| `DiagnosticAuthTransmitCertificate`                     | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticAuthTransmitCertificateEvaluation`           | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticAuthTransmitCertificateMapping`              | [ ] Created | N/A                                      | Group26          |
| `DiagnosticAuthentication`                              | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticAuthenticationClass`                         | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticAuthenticationConfiguration`                 | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticCapabilityElement`                           | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticClearDiagnosticInformation`                  | [ ] Created | N/A                                      | Group24          |
| `DiagnosticClearDiagnosticInformationClass`             | [ ] Created | N/A                                      | Group24          |
| `DiagnosticClearDtcLimitationEnum`                      | [ ] Created | N/A                                      | Group25          |
| `DiagnosticClearDtcNotificationEnum`                    | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticClearEventAllowedBehaviorEnum`               | [ ] Created | N/A                                      | Group25          |
| `DiagnosticClearResetEmissionRelatedInfo`               | [ ] Created | N/A                                      | Group25          |
| `DiagnosticClearResetEmissionRelatedInfoClass`          | [ ] Created | N/A                                      | Group25          |
| `DiagnosticComControl`                                  | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticComControlClass`                             | [ ] Created | N/A                                      | Group24          |
| `DiagnosticComControlSpecificChannel`                   | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticComControlSubNodeChannel`                    | [ ] Created | N/A                                      | Group24          |
| `DiagnosticCommonElement`                               | [ ] Implemented| N/A                                      | Group7           |
| `DiagnosticCommonProps`                                 | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticCommunicationManagerNeeds`                   | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticCompareTypeEnum`                             | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticComponentNeeds`                              | [ ] Implemented| N/A                                      | Group4           |
| `DiagnosticCondition`                                   | [ ] Created | N/A                                      | Group25          |
| `DiagnosticConditionGroup`                              | [ ] Created | N/A                                      | Group25          |
| `DiagnosticConnectedIndicator`                          | [ ] Created | N/A                                      | Group25          |
| `DiagnosticConnectedIndicatorBehaviorEnum`              | [ ] Created | N/A                                      | Group25          |
| `DiagnosticConnection`                                  | [ ] Implemented| N/A                                      | Group5           |
| `DiagnosticContributionSet`                             | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticControlDTCSetting`                           | [ ] Created | N/A                                      | Group24          |
| `DiagnosticControlDTCSettingClass`                      | [ ] Created | N/A                                      | Group24          |
| `DiagnosticControlEnableMaskBit`                        | [ ] Created | N/A                                      | Group24          |
| `DiagnosticControlNeeds`                                | [ ] Implemented| N/A                                      | Group4           |
| `DiagnosticCustomServiceClass`                          | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticCustomServiceInstance`                       | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticDataByIdentifier`                            | [ ] Created | N/A                                      | Group24          |
| `DiagnosticDataElement`                                 | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticDataIdentifier`                              | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticDataIdentifierSet`                           | [ ] Created | N/A                                      | Group25          |
| `DiagnosticDataTransfer`                                | [ ] Created | N/A                                      | Group24          |
| `DiagnosticDataTransferClass`                           | [ ] Created | N/A                                      | Group24          |
| `DiagnosticDeAuthentication`                            | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticDebounceAlgorithmProps`                      | [ ] Created | N/A                                      | Group25          |
| `DiagnosticDebounceBehaviorEnum`                        | [ ] Created | N/A                                      | Group25          |
| `DiagnosticDemProvidedDataMapping`                      | [ ] Created | N/A                                      | Group26          |
| `DiagnosticDenominatorConditionEnum`                    | [ ] Implemented| N/A                                      | Group29          |
| `DiagnosticDynamicDataIdentifier`                       | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticDynamicallyDefineDataIdentifier`             | [ ] Created | N/A                                      | Group24          |
| `DiagnosticDynamicallyDefineDataIdentifierClass`        | [ ] Created | N/A                                      | Group24          |
| `DiagnosticDynamicallyDefineDataIdentifierSubfunctionEnum` | [ ] Created | N/A                                      | Group24          |
| `DiagnosticEcuInstanceProps`                            | [ ] Created | N/A                                      | Group25          |
| `DiagnosticEcuReset`                                    | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticEcuResetClass`                               | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticEnableCondition`                             | [ ] Created | N/A                                      | Group25          |
| `DiagnosticEnableConditionGroup`                        | [ ] Created | N/A                                      | Group25          |
| `DiagnosticEnableConditionNeeds`                        | [ ] Implemented| N/A                                      | Group29          |
| `DiagnosticEnableConditionPortMapping`                  | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEnvBswModeElement`                           | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticEnvCompareCondition`                         | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticEnvConditionFormula`                         | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticEnvConditionFormulaPart`                     | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticEnvDataCondition`                            | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticEnvDataElementCondition`                     | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticEnvModeCondition`                            | [ ] Created | N/A                                      | Group23          |
| `DiagnosticEnvModeElement`                              | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticEnvSwcModeElement`                           | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticEnvironmentalCondition`                      | [ ] Implemented| N/A                                      | Group7           |
| `DiagnosticEvent`                                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticEventClearAllowedEnum`                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticEventCombinationBehaviorEnum`                | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticEventCombinationReportingBehaviorEnum`       | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticEventDisplacementStrategyEnum`               | [ ] Created | N/A                                      | Group25          |
| `DiagnosticEventInfoNeeds`                              | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticEventKindEnum`                               | [ ] Created | N/A                                      | Group25          |
| `DiagnosticEventManagerNeeds`                           | [ ] Implemented| N/A                                      | Group4           |
| `DiagnosticEventNeeds`                                  | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticEventPortMapping`                            | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEventToDebounceAlgorithmMapping`             | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEventToEnableConditionGroupMapping`          | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEventToOperationCycleMapping`                | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEventToSecurityEventMapping`                 | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEventToStorageConditionGroupMapping`         | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEventToTroubleCodeJ1939Mapping`              | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEventToTroubleCodeUdsMapping`                | [ ] Created | N/A                                      | Group26          |
| `DiagnosticEventWindow`                                 | [ ] Created | N/A                                      | Group24          |
| `DiagnosticEventWindowTimeEnum`                         | [ ] Created | N/A                                      | Group24          |
| `DiagnosticExtendedDataRecord`                          | [ ] Created | N/A                                      | Group25          |
| `DiagnosticFimAliasEvent`                               | [ ] Created | N/A                                      | Group25          |
| `DiagnosticFimAliasEventGroup`                          | [ ] Created | N/A                                      | Group26          |
| `DiagnosticFimAliasEventGroupMapping`                   | [ ] Created | N/A                                      | Group26          |
| `DiagnosticFimAliasEventMapping`                        | [ ] Created | N/A                                      | Group26          |
| `DiagnosticFimEventGroup`                               | [ ] Created | N/A                                      | Group26          |
| `DiagnosticFimFunctionMapping`                          | [ ] Created | N/A                                      | Group26          |
| `DiagnosticFreezeFrame`                                 | [ ] Created | N/A                                      | Group25          |
| `DiagnosticFunctionIdentifier`                          | [ ] Created | N/A                                      | Group25          |
| `DiagnosticFunctionIdentifierInhibit`                   | [ ] Created | N/A                                      | Group25          |
| `DiagnosticFunctionInhibitSource`                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticHandleDDDIConfigurationEnum`                 | [ ] Created | N/A                                      | Group24          |
| `DiagnosticIOControl`                                   | [ ] Created | N/A                                      | Group24          |
| `DiagnosticIndicator`                                   | [ ] Created | N/A                                      | Group25          |
| `DiagnosticIndicatorTypeEnum`                           | [ ] Implemented| N/A                                      | Group29          |
| `DiagnosticInfoType`                                    | [ ] Created | N/A                                      | Group25          |
| `DiagnosticInhibitSourceEventMapping`                   | [ ] Created | N/A                                      | Group26          |
| `DiagnosticInhibitionMaskEnum`                          | [ ] Created | N/A                                      | Group26          |
| `DiagnosticIoControlClass`                              | [ ] Created | N/A                                      | Group24          |
| `DiagnosticIoControlNeeds`                              | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticIumpr`                                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticIumprDenominatorGroup`                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticIumprGroup`                                  | [ ] Created | N/A                                      | Group25          |
| `DiagnosticIumprGroupIdentifier`                        | [ ] Created | N/A                                      | Group25          |
| `DiagnosticIumprKindEnum`                               | [ ] Created | N/A                                      | Group25          |
| `DiagnosticIumprToFunctionIdentifierMapping`            | [ ] Created | N/A                                      | Group26          |
| `DiagnosticJ1939ExpandedFreezeFrame`                    | [ ] Created | N/A                                      | Group26          |
| `DiagnosticJ1939FreezeFrame`                            | [ ] Created | N/A                                      | Group26          |
| `DiagnosticJ1939Node`                                   | [ ] Created | N/A                                      | Group26          |
| `DiagnosticJ1939Spn`                                    | [ ] Created | N/A                                      | Group26          |
| `DiagnosticJ1939SpnMapping`                             | [ ] Created | N/A                                      | Group26          |
| `DiagnosticJ1939SwMapping`                              | [ ] Created | N/A                                      | Group26          |
| `DiagnosticJumpToBootLoaderEnum`                        | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticLogicalOperatorEnum`                         | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticMapping`                                     | [ ] Implemented| N/A                                      | Group26          |
| `DiagnosticMasterToSlaveEventMapping`                   | [ ] Created | N/A                                      | Group26          |
| `DiagnosticMeasurementIdentifier`                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticMemoryAddressableRangeAccess`                | [ ] Created | N/A                                      | Group24          |
| `DiagnosticMemoryByAddress`                             | [ ] Created | N/A                                      | Group24          |
| `DiagnosticMemoryDestination`                           | [ ] Created | N/A                                      | Group25          |
| `DiagnosticMemoryDestinationPrimary`                    | [ ] Created | N/A                                      | Group25          |
| `DiagnosticMemoryDestinationUserDefined`                | [ ] Created | N/A                                      | Group25          |
| `DiagnosticMemoryEntryStorageTriggerEnum`               | [ ] Created | N/A                                      | Group25          |
| `DiagnosticMemoryIdentifier`                            | [ ] Created | N/A                                      | Group24          |
| `DiagnosticMonitorUpdateKindEnum`                       | [ ] Implemented| N/A                                      | Group29          |
| `DiagnosticObdSupportEnum`                              | [ ] Created | N/A                                      | Group25          |
| `DiagnosticOccurrenceCounterProcessingEnum`             | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticOperationCycle`                              | [ ] Created | N/A                                      | Group25          |
| `DiagnosticOperationCycleNeeds`                         | [ ] Implemented| N/A                                      | Group29          |
| `DiagnosticOperationCyclePortMapping`                   | [ ] Created | N/A                                      | Group26          |
| `DiagnosticOperationCycleTypeEnum`                      | [ ] Created | N/A                                      | Group25          |
| `DiagnosticParameter`                                   | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticParameterElement`                            | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticParameterElementAccess`                      | [ ] Created | N/A                                      | Group26          |
| `DiagnosticParameterIdent`                              | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticParameterIdentifier`                         | [ ] Created | N/A                                      | Group24          |
| `DiagnosticParameterSupportInfo`                        | [ ] Created | N/A                                      | Group24          |
| `DiagnosticPeriodicRate`                                | [ ] Created | N/A                                      | Group24          |
| `DiagnosticPeriodicRateCategoryEnum`                    | [ ] Created | N/A                                      | Group24          |
| `DiagnosticPowertrainFreezeFrame`                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticProcessingStyleEnum`                         | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticProofOfOwnership`                            | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticProtocol`                                    | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticReadDTCInformation`                          | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadDTCInformationClass`                     | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadDataByIdentifier`                        | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadDataByIdentifierClass`                   | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadDataByPeriodicID`                        | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadDataByPeriodicIDClass`                   | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadMemoryByAddress`                         | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadMemoryByAddressClass`                    | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadScalingDataByIdentifier`                 | [ ] Created | N/A                                      | Group24          |
| `DiagnosticReadScalingDataByIdentifierClass`            | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRecordTriggerEnum`                           | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestControlOfOnBoardDevice`               | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestControlOfOnBoardDeviceClass`          | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestCurrentPowertrainData`                | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestCurrentPowertrainDataClass`           | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestDownload`                             | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRequestDownloadClass`                        | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRequestEmissionRelatedDTC`                   | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestEmissionRelatedDTCClass`              | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestEmissionRelatedDTCPermanentStatus`    | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestEmissionRelatedDTCPermanentStatusClass` | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestFileTransfer`                         | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRequestFileTransferClass`                    | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRequestFileTransferNeeds`                    | [ ] Implemented| N/A                                      | Group4           |
| `DiagnosticRequestOnBoardMonitoringTestResults`         | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestOnBoardMonitoringTestResultsClass`    | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestPowertrainFreezeFrameData`            | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestPowertrainFreezeFrameDataClass`       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestRoutineResults`                       | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRequestUpload`                               | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRequestUploadClass`                          | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRequestVehicleInfo`                          | [ ] Created | N/A                                      | Group25          |
| `DiagnosticRequestVehicleInfoClass`                     | [ ] Created | N/A                                      | Group25          |
| `DiagnosticResponseOnEvent`                             | [ ] Created | N/A                                      | Group24          |
| `DiagnosticResponseOnEventActionEnum`                   | [ ] Created | N/A                                      | Group24          |
| `DiagnosticResponseOnEventClass`                        | [ ] Created | N/A                                      | Group24          |
| `DiagnosticResponseToEcuResetEnum`                      | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticRoutine`                                     | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRoutineControl`                              | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRoutineControlClass`                         | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRoutineNeeds`                                | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticRoutineSubfunction`                          | [ ] Created | N/A                                      | Group24          |
| `DiagnosticRoutineTypeEnum`                             | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticSecurityAccess`                              | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticSecurityAccessClass`                         | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticSecurityEventReportingModeMapping`           | [ ] Created | N/A                                      | Group26          |
| `DiagnosticSecurityLevel`                               | [ ] Implemented| N/A                                      | Group7           |
| `DiagnosticServiceClass`                                | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticServiceDataMapping`                          | [ ] Created | N/A                                      | Group26          |
| `DiagnosticServiceInstance`                             | [ ] Implemented| N/A                                      | Group7           |
| `DiagnosticServiceMappingDiagTarget`                    | [ ] Created | N/A                                      | Group26          |
| `DiagnosticServiceRequestCallbackTypeEnum`              | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticServiceSwMapping`                            | [ ] Created | N/A                                      | Group26          |
| `DiagnosticServiceTable`                                | [ ] Implemented| N/A                                      | Group7           |
| `DiagnosticSession`                                     | [ ] Implemented| N/A                                      | Group7           |
| `DiagnosticSessionControl`                              | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticSessionControlClass`                         | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticSignificanceEnum`                            | [ ] Created | N/A                                      | Group25          |
| `DiagnosticStartRoutine`                                | [ ] Created | N/A                                      | Group24          |
| `DiagnosticStatusBitHandlingTestFailedSinceLastClearEnum` | [ ] Created | N/A                                      | Group25          |
| `DiagnosticStopRoutine`                                 | [ ] Created | N/A                                      | Group24          |
| `DiagnosticStorageCondition`                            | [ ] Created | N/A                                      | Group25          |
| `DiagnosticStorageConditionGroup`                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticStorageConditionNeeds`                       | [ ] Implemented| N/A                                      | Group29          |
| `DiagnosticStorageConditionPortMapping`                 | [ ] Created | N/A                                      | Group26          |
| `DiagnosticSupportInfoByte`                             | [ ] Created | N/A                                      | Group25          |
| `DiagnosticSwMapping`                                   | [ ] Created | N/A                                      | Group26          |
| `DiagnosticTestIdentifier`                              | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTestResult`                                  | [ ] Created | N/A                                      | Group29          |
| `DiagnosticTestResultUpdateEnum`                        | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTestRoutineIdentifier`                       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTransferExit`                                | [ ] Created | N/A                                      | Group24          |
| `DiagnosticTransferExitClass`                           | [ ] Created | N/A                                      | Group24          |
| `DiagnosticTroubleCode`                                 | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTroubleCodeGroup`                            | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTroubleCodeJ1939`                            | [ ] Created | N/A                                      | Group26          |
| `DiagnosticTroubleCodeJ1939DtcKindEnum`                 | [ ] Created | N/A                                      | Group26          |
| `DiagnosticTroubleCodeObd`                              | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTroubleCodeProps`                            | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTroubleCodeUds`                              | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTroubleCodeUdsToTroubleCodeObdMapping`       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticTypeOfDtcSupportedEnum`                      | [ ] Implemented| N/A                                      | Group23          |
| `DiagnosticTypeOfFreezeFrameRecordNumerationEnum`       | [ ] Created | N/A                                      | Group25          |
| `DiagnosticUdsSeverityEnum`                             | [ ] Created | N/A                                      | Group25          |
| `DiagnosticUploadDownloadNeeds`                         | [ ] Implemented| N/A                                      | Group4           |
| `DiagnosticValueAccessEnum`                             | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticValueNeeds`                                  | [ ] Implemented| N/A                                      | Group14          |
| `DiagnosticVerifyCertificateBidirectional`              | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticVerifyCertificateUnidirectional`             | [ ] Implemented| N/A                                      | Group24          |
| `DiagnosticWriteDataByIdentifier`                       | [ ] Created | N/A                                      | Group24          |
| `DiagnosticWriteDataByIdentifierClass`                  | [ ] Created | N/A                                      | Group24          |
| `DiagnosticWriteMemoryByAddress`                        | [ ] Created | N/A                                      | Group24          |
| `DiagnosticWriteMemoryByAddressClass`                   | [ ] Created | N/A                                      | Group24          |
| `DiagnosticWwhObdDtcClassEnum`                          | [ ] Created | N/A                                      | Group25          |
| `DiagnosticsCommunicationSecurityNeeds`                 | [ ] Implemented| N/A                                      | Group4           |
| `DisplayPresentationEnum`                               | [ ] Implemented| N/A                                      | Group28          |
| `DltApplication`                                        | [ ] Implemented| N/A                                      | Group5           |
| `DltArgument`                                           | [ ] Implemented| N/A                                      | Group5           |
| `DltConfig`                                             | [ ] Implemented| N/A                                      | Group5           |
| `DltContext`                                            | [ ] Implemented| N/A                                      | Group5           |
| `DltDefaultTraceStateEnum`                              | [ ] Implemented| N/A                                      | Group5           |
| `DltEcu`                                                | [ ] Implemented| N/A                                      | Group5           |
| `DltLogChannel`                                         | [ ] Implemented| N/A                                      | Group5           |
| `DltMessage`                                            | [ ] Implemented| N/A                                      | Group5           |
| `DltUserNeeds`                                          | [ ] Implemented| N/A                                      | Group4           |
| `DoIpActivationLineNeeds`                               | [ ] Implemented| N/A                                      | Group4           |
| `DoIpConfig`                                            | [ ] Implemented| N/A                                      | Group5           |
| `DoIpEntity`                                            | [ ] Implemented| N/A                                      | Group16          |
| `DoIpEntityRoleEnum`                                    | [ ] Implemented| N/A                                      | Group32          |
| `DoIpGidNeeds`                                          | [ ] Implemented| N/A                                      | Group4           |
| `DoIpGidSynchronizationNeeds`                           | [ ] Implemented| N/A                                      | Group4           |
| `DoIpInterface`                                         | [ ] Implemented| N/A                                      | Group5           |
| `DoIpLogicAddress`                                      | [ ] Implemented| N/A                                      | Group20          |
| `DoIpLogicTargetAddressProps`                           | [ ] Implemented| N/A                                      | Group7           |
| `DoIpLogicTesterAddressProps`                           | [ ] Implemented| N/A                                      | Group7           |
| `DoIpPowerModeStatusNeeds`                              | [ ] Implemented| N/A                                      | Group5           |
| `DoIpRoutingActivation`                                 | [ ] Implemented| N/A                                      | Group5           |
| `DoIpRoutingActivationAuthenticationNeeds`              | [ ] Implemented| N/A                                      | Group29          |
| `DoIpRoutingActivationConfirmationNeeds`                | [ ] Implemented| N/A                                      | Group29          |
| `DoIpRule`                                              | [ ] Implemented| N/A                                      | Group20          |
| `DoIpServiceNeeds`                                      | [ ] Implemented| N/A                                      | Group23          |
| `DoIpTpConfig`                                          | [ ] Implemented| N/A                                      | Group7           |
| `DoIpTpConnection`                                      | [ ] Implemented| N/A                                      | Group20          |
| `DocumentElementScope`                                  | [ ] Created | N/A                                      | Group36          |
| `DocumentViewSelectable`                                | [ ] Implemented| N/A                                      | Group3           |
| `DocumentationBlock`                                    | [ ] Implemented| N/A                                      | Group21          |
| `DocumentationContext`                                  | [ ] Implemented| N/A                                      | Group21          |
| `DtcFormatTypeEnum`                                     | [ ] Implemented| N/A                                      | Group14          |
| `DtcKindEnum`                                           | [ ] Implemented| N/A                                      | Group14          |
| `DtcStatusChangeNotificationNeeds`                      | [ ] Implemented| N/A                                      | Group14          |
| `DynamicPart`                                           | [ ] Implemented| N/A                                      | Group15          |
| `DynamicPartAlternative`                                | [ ] Implemented| N/A                                      | Group5           |
| `E2EProfileCompatibilityProps`                          | [ ] Implemented| N/A                                      | Group28          |
| `ECUMapping`                                            | [ ] Implemented| N/A                                      | Group7           |
| `EEnum`                                                 | [ ] Implemented| N/A                                      | Group21          |
| `EEnumFont`                                             | [ ] Implemented| N/A                                      | Group21          |
| `EOCEventRef`                                           | [ ] Implemented| N/A                                      | Group35          |
| `EOCExecutableEntityRef`                                | [ ] Implemented| N/A                                      | Group35          |
| `EOCExecutableEntityRefAbstract`                        | [ ] Implemented| N/A                                      | Group35          |
| `EOCExecutableEntityRefGroup`                           | [ ] Implemented| N/A                                      | Group35          |
| `EcuAbstractionSwComponentType`                         | [ ] Implemented| N/A                                      | Group29          |
| `EcuInstance`                                           | [ ] Implemented| N/A                                      | Group5           |
| `EcuPartition`                                          | [ ] Implemented| N/A                                      | Group5           |
| `EcuResourceEstimation`                                 | [ ] Created | N/A                                      | Group31          |
| `EcuStateMgrUserNeeds`                                  | [ ] Implemented| N/A                                      | Group4           |
| `EcuTiming`                                             | [ ] Created | N/A                                      | Group35          |
| `EcucAbstractConfigurationClass`                        | [ ] Implemented| N/A                                      | Group26          |
| `EcucAbstractExternalReferenceDef`                      | [ ] Implemented| N/A                                      | Group26          |
| `EcucAbstractInternalReferenceDef`                      | [ ] Implemented| N/A                                      | Group26          |
| `EcucAbstractReferenceDef`                              | [ ] Implemented| N/A                                      | Group26          |
| `EcucAbstractReferenceValue`                            | [ ] Implemented| N/A                                      | Group27          |
| `EcucAbstractStringParamDef`                            | [ ] Implemented| N/A                                      | Group26          |
| `EcucAddInfoParamDef`                                   | [ ] Implemented| N/A                                      | Group26          |
| `EcucAddInfoParamValue`                                 | [ ] Implemented| N/A                                      | Group27          |
| `EcucBooleanParamDef`                                   | [ ] Implemented| N/A                                      | Group19          |
| `EcucChoiceContainerDef`                                | [ ] Implemented| N/A                                      | Group26          |
| `EcucChoiceReferenceDef`                                | [ ] Implemented| N/A                                      | Group26          |
| `EcucCommonAttributes`                                  | [ ] Implemented| N/A                                      | Group26          |
| `EcucConditionFormula`                                  | [ ] Implemented| N/A                                      | Group19          |
| `EcucConditionSpecification`                            | [ ] Implemented| N/A                                      | Group27          |
| `EcucConfigurationClassEnum`                            | [ ] Implemented| N/A                                      | Group19          |
| `EcucConfigurationVariantEnum`                          | [ ] Implemented| N/A                                      | Group26          |
| `EcucContainerDef`                                      | [ ] Implemented| N/A                                      | Group26          |
| `EcucContainerValue`                                    | [ ] Implemented| N/A                                      | Group27          |
| `EcucDefinitionCollection`                              | [ ] Implemented| N/A                                      | Group26          |
| `EcucDefinitionElement`                                 | [ ] Implemented| N/A                                      | Group26          |
| `EcucDerivationSpecification`                           | [ ] Implemented| N/A                                      | Group26          |
| `EcucDestinationUriDef`                                 | [ ] Implemented| N/A                                      | Group26          |
| `EcucDestinationUriDefRefType`                          | [ ] Implemented| N/A                                      | Group19          |
| `EcucDestinationUriDefSet`                              | [ ] Implemented| N/A                                      | Group26          |
| `EcucDestinationUriNestingContractEnum`                 | [ ] Implemented| N/A                                      | Group26          |
| `EcucDestinationUriPolicy`                              | [ ] Implemented| N/A                                      | Group26          |
| `EcucEnumerationLiteralDef`                             | [ ] Implemented| N/A                                      | Group26          |
| `EcucEnumerationParamDef`                               | [ ] Implemented| N/A                                      | Group26          |
| `EcucFloatParamDef`                                     | [ ] Implemented| N/A                                      | Group19          |
| `EcucForeignReferenceDef`                               | [ ] Implemented| N/A                                      | Group19          |
| `EcucFunctionNameDef`                                   | [ ] Implemented| N/A                                      | Group26          |
| `EcucIndexableValue`                                    | [ ] Implemented| N/A                                      | Group27          |
| `EcucInstanceReferenceDef`                              | [ ] Implemented| N/A                                      | Group26          |
| `EcucInstanceReferenceValue`                            | [ ] Implemented| N/A                                      | Group27          |
| `EcucIntegerParamDef`                                   | [ ] Implemented| N/A                                      | Group26          |
| `EcucLinkerSymbolDef`                                   | [ ] Implemented| N/A                                      | Group19          |
| `EcucModuleConfigurationValues`                         | [ ] Implemented| N/A                                      | Group27          |
| `EcucModuleDef`                                         | [ ] Implemented| N/A                                      | Group26          |
| `EcucMultilineStringParamDef`                           | [ ] Implemented| N/A                                      | Group26          |
| `EcucMultiplicityConfigurationClass`                    | [ ] Implemented| N/A                                      | Group26          |
| `EcucNumericalParamValue`                               | [ ] Implemented| N/A                                      | Group27          |
| `EcucParamConfContainerDef`                             | [ ] Implemented| N/A                                      | Group26          |
| `EcucParameterDef`                                      | [ ] Implemented| N/A                                      | Group26          |
| `EcucParameterDerivationFormula`                        | [ ] Implemented| N/A                                      | Group19          |
| `EcucParameterValue`                                    | [ ] Implemented| N/A                                      | Group27          |
| `EcucQuery`                                             | [ ] Implemented| N/A                                      | Group26          |
| `EcucQueryExpression`                                   | [ ] Implemented| N/A                                      | Group19          |
| `EcucReferenceDef`                                      | [ ] Implemented| N/A                                      | Group19          |
| `EcucReferenceValue`                                    | [ ] Implemented| N/A                                      | Group27          |
| `EcucScopeEnum`                                         | [ ] Implemented| N/A                                      | Group19          |
| `EcucStringParamDef`                                    | [ ] Implemented| N/A                                      | Group26          |
| `EcucSymbolicNameReferenceDef`                          | [ ] Implemented| N/A                                      | Group19          |
| `EcucTextualParamValue`                                 | [ ] Implemented| N/A                                      | Group27          |
| `EcucUriReferenceDef`                                   | [ ] Implemented| N/A                                      | Group19          |
| `EcucValidationCondition`                               | [ ] Implemented| N/A                                      | Group27          |
| `EcucValueCollection`                                   | [ ] Implemented| N/A                                      | Group19          |
| `EcucValueConfigurationClass`                           | [ ] Implemented| N/A                                      | Group26          |
| `EmphasisText`                                          | [ ] Implemented| N/A                                      | Group21          |
| `EndToEndDescription`                                   | [ ] Implemented| N/A                                      | Group10          |
| `EndToEndProfileBehaviorEnum`                           | [ ] Implemented| N/A                                      | Group34          |
| `EndToEndProtection`                                    | [ ] Implemented| N/A                                      | Group28          |
| `EndToEndProtectionISignalIPdu`                         | [ ] Implemented| N/A                                      | Group18          |
| `EndToEndProtectionSet`                                 | [ ] Implemented| N/A                                      | Group5           |
| `EndToEndProtectionVariablePrototype`                   | [ ] Implemented| N/A                                      | Group5           |
| `EndToEndTransformationComSpecProps`                    | [ ] Implemented| N/A                                      | Group28          |
| `EndToEndTransformationDescription`                     | [ ] Implemented| N/A                                      | Group34          |
| `EndToEndTransformationISignalProps`                    | [ ] Implemented| N/A                                      | Group18          |
| `Entry`                                                 | [ ] Implemented| N/A                                      | Group3           |
| `ErrorTracerNeeds`                                      | [ ] Implemented| N/A                                      | Group23          |
| `EthGlobalTimeDomainProps`                              | [ ] Created | N/A                                      | Group34          |
| `EthGlobalTimeManagedCouplingPort`                      | [ ] Created | N/A                                      | Group34          |
| `EthGlobalTimeMessageFormatEnum`                        | [ ] Created | N/A                                      | Group34          |
| `EthIpProps`                                            | [ ] Created | N/A                                      | Group30          |
| `EthTSynCrcFlags`                                       | [ ] Created | N/A                                      | Group34          |
| `EthTSynSubTlvConfig`                                   | [ ] Created | N/A                                      | Group34          |
| `EthTcpIpIcmpProps`                                     | [ ] Implemented| N/A                                      | Group5           |
| `EthTcpIpProps`                                         | [ ] Implemented| N/A                                      | Group5           |
| `EthTpConfig`                                           | [ ] Created | N/A                                      | Group33          |
| `EthTpConnection`                                       | [ ] Created | N/A                                      | Group33          |
| `EthernetCluster`                                       | [ ] Implemented| N/A                                      | Group29          |
| `EthernetCommunicationConnector`                        | [ ] Implemented| N/A                                      | Group30          |
| `EthernetCommunicationController`                       | [ ] Implemented| N/A                                      | Group30          |
| `EthernetConnectionNegotiationEnum`                     | [ ] Implemented| N/A                                      | Group30          |
| `EthernetCouplingPortSchedulerEnum`                     | [ ] Implemented| N/A                                      | Group30          |
| `EthernetFrameTriggering`                               | [ ] Created | N/A                                      | Group33          |
| `EthernetMacLayerTypeEnum`                              | [ ] Implemented| N/A                                      | Group30          |
| `EthernetPhysicalChannel`                               | [ ] Implemented| N/A                                      | Group5           |
| `EthernetPhysicalLayerTypeEnum`                         | [ ] Implemented| N/A                                      | Group30          |
| `EthernetPriorityRegeneration`                          | [ ] Implemented| N/A                                      | Group16          |
| `EthernetSwitchVlanEgressTaggingEnum`                   | [ ] Implemented| N/A                                      | Group30          |
| `EthernetSwitchVlanIngressTagEnum`                      | [ ] Implemented| N/A                                      | Group30          |
| `EthernetWakeupSleepOnDatalineConfig`                   | [ ] Created | N/A                                      | Group30          |
| `EthernetWakeupSleepOnDatalineConfigSet`                | [ ] Created | N/A                                      | Group30          |
| `EvaluatedVariantSet`                                   | [ ] Implemented| N/A                                      | Group21          |
| `EventAcceptanceStatusEnum`                             | [ ] Implemented| N/A                                      | Group29          |
| `EventControlledTiming`                                 | [ ] Implemented| N/A                                      | Group15          |
| `EventGroupControlTypeEnum`                             | [ ] Implemented| N/A                                      | Group32          |
| `EventHandler`                                          | [ ] Implemented| N/A                                      | Group32          |
| `EventObdReadinessGroup`                                | [ ] Created | N/A                                      | Group25          |
| `EventOccurrenceKindEnum`                               | [ ] Implemented| N/A                                      | Group35          |
| `EventTriggeringConstraint`                             | [ ] Implemented| N/A                                      | Group35          |
| `ExclusiveArea`                                         | [ ] Implemented| N/A                                      | Group22          |
| `ExclusiveAreaNestingOrder`                             | [ ] Implemented| N/A                                      | Group22          |
| `ExecutableEntity`                                      | [ ] Implemented| N/A                                      | Group22          |
| `ExecutableEntityActivationReason`                      | [ ] Implemented| N/A                                      | Group28          |
| `ExecutionOrderConstraint`                              | [ ] Implemented| N/A                                      | Group35          |
| `ExecutionOrderConstraintTypeEnum`                      | [ ] Implemented| N/A                                      | Group35          |
| `ExecutionTime`                                         | [ ] Implemented| N/A                                      | Group23          |
| `ExecutionTimeConstraint`                               | [ ] Implemented| N/A                                      | Group35          |
| `ExecutionTimeTypeEnum`                                 | [ ] Implemented| N/A                                      | Group35          |
| `ExternalTriggerOccurredEvent`                          | [ ] Created | N/A                                      | Group28          |
| `ExternalTriggeringPoint`                               | [ ] Implemented| N/A                                      | Group29          |
| `ExternalTriggeringPointIdent`                          | [ ] Implemented| N/A                                      | Group2           |
| `FMAttributeDef`                                        | [ ] Created | N/A                                      | Group36          |
| `FMAttributeValue`                                      | [ ] Created | N/A                                      | Group36          |
| `FMConditionByFeaturesAndAttributes`                    | [ ] Implemented| N/A                                      | Group8           |
| `FMConditionByFeaturesAndSwSystemconsts`                | [ ] Implemented| N/A                                      | Group8           |
| `FMFeature`                                             | [ ] Created | N/A                                      | Group36          |
| `FMFeatureDecomposition`                                | [ ] Created | N/A                                      | Group36          |
| `FMFeatureMap`                                          | [ ] Created | N/A                                      | Group36          |
| `FMFeatureMapAssertion`                                 | [ ] Created | N/A                                      | Group36          |
| `FMFeatureMapCondition`                                 | [ ] Created | N/A                                      | Group36          |
| `FMFeatureMapElement`                                   | [ ] Created | N/A                                      | Group36          |
| `FMFeatureModel`                                        | [ ] Created | N/A                                      | Group36          |
| `FMFeatureRelation`                                     | [ ] Created | N/A                                      | Group36          |
| `FMFeatureRestriction`                                  | [ ] Created | N/A                                      | Group36          |
| `FMFeatureSelection`                                    | [ ] Created | N/A                                      | Group36          |
| `FMFeatureSelectionSet`                                 | [ ] Created | N/A                                      | Group36          |
| `FMFeatureSelectionState`                               | [ ] Created | N/A                                      | Group36          |
| `FMFormulaByFeaturesAndAttributes`                      | [ ] Implemented| N/A                                      | Group8           |
| `FMFormulaByFeaturesAndSwSystemconsts`                  | [ ] Implemented| N/A                                      | Group8           |
| `Field`                                                 | [ ] Implemented| N/A                                      | Group11          |
| `FileInfoComment`                                       | [ ] Implemented| N/A                                      | Group1           |
| `FilterDebouncingEnum`                                  | [ ] Implemented| N/A                                      | Group27          |
| `FirewallActionEnum`                                    | [ ] Implemented| N/A                                      | Group3           |
| `FirewallRule`                                          | [ ] Implemented| N/A                                      | Group1           |
| `FirewallRuleProps`                                     | [ ] Implemented| N/A                                      | Group7           |
| `FlatInstanceDescriptor`                                | [ ] Implemented| N/A                                      | Group1           |
| `FlatMap`                                               | [ ] Implemented| N/A                                      | Group1           |
| `FlexrayAbsolutelyScheduledTiming`                      | [ ] Implemented| N/A                                      | Group17          |
| `FlexrayArTpChannel`                                    | [ ] Created | N/A                                      | Group33          |
| `FlexrayArTpConfig`                                     | [ ] Created | N/A                                      | Group33          |
| `FlexrayArTpConnection`                                 | [ ] Created | N/A                                      | Group33          |
| `FlexrayArTpNode`                                       | [ ] Created | N/A                                      | Group33          |
| `FlexrayChannelName`                                    | [ ] Implemented| N/A                                      | Group15          |
| `FlexrayCluster`                                        | [ ] Implemented| N/A                                      | Group29          |
| `FlexrayCommunicationConnector`                         | [ ] Implemented| N/A                                      | Group17          |
| `FlexrayCommunicationController`                        | [ ] Implemented| N/A                                      | Group17          |
| `FlexrayFifoConfiguration`                              | [ ] Implemented| N/A                                      | Group29          |
| `FlexrayFifoRange`                                      | [ ] Implemented| N/A                                      | Group29          |
| `FlexrayFrame`                                          | [ ] Implemented| N/A                                      | Group6           |
| `FlexrayFrameTriggering`                                | [ ] Implemented| N/A                                      | Group17          |
| `FlexrayNmCluster`                                      | [ ] Implemented| N/A                                      | Group6           |
| `FlexrayNmClusterCoupling`                              | [ ] Implemented| N/A                                      | Group18          |
| `FlexrayNmEcu`                                          | [ ] Implemented| N/A                                      | Group6           |
| `FlexrayNmNode`                                         | [ ] Implemented| N/A                                      | Group6           |
| `FlexrayNmScheduleVariant`                              | [ ] Implemented| N/A                                      | Group33          |
| `FlexrayPhysicalChannel`                                | [ ] Implemented| N/A                                      | Group17          |
| `FlexrayTpConfig`                                       | [ ] Created | N/A                                      | Group33          |
| `FlexrayTpConnection`                                   | [ ] Created | N/A                                      | Group33          |
| `FlexrayTpConnectionControl`                            | [ ] Created | N/A                                      | Group33          |
| `FlexrayTpEcu`                                          | [ ] Created | N/A                                      | Group33          |
| `FlexrayTpNode`                                         | [ ] Created | N/A                                      | Group33          |
| `FlexrayTpPduPool`                                      | [ ] Created | N/A                                      | Group33          |
| `FloatEnum`                                             | [ ] Implemented| N/A                                      | Group22          |
| `FloatValueVariationPoint`                              | [ ] Implemented| N/A                                      | Group8           |
| `FlowMeteringColorModeEnum`                             | [ ] Created | N/A                                      | Group30          |
| `ForbiddenSignalPath`                                   | [ ] Created | N/A                                      | Group31          |
| `FormulaExpression`                                     | [ ] Implemented| N/A                                      | Group8           |
| `FrArTpAckType`                                         | [ ] Created | N/A                                      | Group33          |
| `FrGlobalTimeDomainProps`                               | [ ] Created | N/A                                      | Group34          |
| `Frame`                                                 | [ ] Implemented| N/A                                      | Group31          |
| `FrameEnum`                                             | [ ] Implemented| N/A                                      | Group3           |
| `FrameMapping`                                          | [ ] Implemented| N/A                                      | Group17          |
| `FramePid`                                              | [ ] Implemented| N/A                                      | Group31          |
| `FramePort`                                             | [ ] Implemented| N/A                                      | Group5           |
| `FrameTriggering`                                       | [ ] Implemented| N/A                                      | Group5           |
| `FreeFormat`                                            | [ ] Implemented| N/A                                      | Group32          |
| `FreeFormatEntry`                                       | [ ] Implemented| N/A                                      | Group31          |
| `FullBindingTimeEnum`                                   | [ ] Implemented| N/A                                      | Group21          |
| `FunctionInhibitionAvailabilityNeeds`                   | [ ] Implemented| N/A                                      | Group29          |
| `FunctionInhibitionNeeds`                               | [ ] Implemented| N/A                                      | Group4           |
| `FurtherActionByteNeeds`                                | [ ] Implemented| N/A                                      | Group5           |
| `Gateway`                                               | [ ] Implemented| N/A                                      | Group17          |
| `GeneralAnnotation`                                     | [ ] Implemented| N/A                                      | Group3           |
| `GeneralParameter`                                      | [ ] Implemented| N/A                                      | Group9           |
| `GeneralPurposeConnection`                              | [ ] Created | N/A                                      | Group31          |
| `GeneralPurposeIPdu`                                    | [ ] Implemented| N/A                                      | Group5           |
| `GeneralPurposePdu`                                     | [ ] Implemented| N/A                                      | Group5           |
| `GenericEthernetFrame`                                  | [ ] Implemented| N/A                                      | Group6           |
| `GenericTp`                                             | [ ] Implemented| N/A                                      | Group16          |
| `GlobalSupervisionNeeds`                                | [ ] Implemented| N/A                                      | Group4           |
| `GlobalTimeCanMaster`                                   | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeCanSlave`                                    | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeCorrectionProps`                             | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeCouplingPortProps`                           | [ ] Implemented| N/A                                      | Group34          |
| `GlobalTimeCrcSupportEnum`                              | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeCrcValidationEnum`                           | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeDomain`                                      | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeEthMaster`                                   | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeEthSlave`                                    | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeFrMaster`                                    | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeFrSlave`                                     | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeGateway`                                     | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeIcvSupportEnum`                              | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeIcvVerificationEnum`                         | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeMaster`                                      | [ ] Created | N/A                                      | Group34          |
| `GlobalTimePortRoleEnum`                                | [ ] Created | N/A                                      | Group34          |
| `GlobalTimeSlave`                                       | [ ] Created | N/A                                      | Group34          |
| `Graphic`                                               | [ ] Implemented| N/A                                      | Group3           |
| `GraphicFitEnum`                                        | [ ] Implemented| N/A                                      | Group3           |
| `GraphicNotationEnum`                                   | [ ] Implemented| N/A                                      | Group3           |
| `HandleInvalidEnum`                                     | [ ] Implemented| N/A                                      | Group1           |
| `HandleOutOfRangeEnum`                                  | [ ] Implemented| N/A                                      | Group27          |
| `HandleOutOfRangeStatusEnum`                            | [ ] Implemented| N/A                                      | Group27          |
| `HandleTimeoutEnum`                                     | [ ] Implemented| N/A                                      | Group27          |
| `HardwareConfiguration`                                 | [ ] Implemented| N/A                                      | Group20          |
| `HardwareTestNeeds`                                     | [ ] Implemented| N/A                                      | Group4           |
| `HeapUsage`                                             | [ ] Implemented| N/A                                      | Group22          |
| `HttpTp`                                                | [ ] Created | N/A                                      | Group32          |
| `HwAttributeDef`                                        | [ ] Implemented| N/A                                      | Group7           |
| `HwAttributeLiteralDef`                                 | [ ] Implemented| N/A                                      | Group7           |
| `HwAttributeValue`                                      | [ ] Implemented| N/A                                      | Group7           |
| `HwCategory`                                            | [ ] Implemented| N/A                                      | Group7           |
| `HwDescriptionEntity`                                   | [ ] Implemented| N/A                                      | Group27          |
| `HwElement`                                             | [ ] Implemented| N/A                                      | Group1           |
| `HwElementConnector`                                    | [ ] Implemented| N/A                                      | Group27          |
| `HwPin`                                                 | [ ] Implemented| N/A                                      | Group1           |
| `HwPinConnector`                                        | [ ] Implemented| N/A                                      | Group27          |
| `HwPinGroup`                                            | [ ] Implemented| N/A                                      | Group1           |
| `HwPinGroupConnector`                                   | [ ] Implemented| N/A                                      | Group27          |
| `HwPinGroupContent`                                     | [ ] Implemented| N/A                                      | Group27          |
| `HwPortMapping`                                         | [ ] Implemented| N/A                                      | Group7           |
| `HwType`                                                | [ ] Implemented| N/A                                      | Group1           |
| `IEEE1722TpAafAes3DataTypeEnum`                         | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAafConnection`                               | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAafFormatEnum`                               | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAafNominalRateEnum`                          | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAcfBus`                                      | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAcfBusPart`                                  | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAcfCan`                                      | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAcfCanMessageTypeEnum`                       | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAcfCanPart`                                  | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAcfConnection`                               | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAcfLin`                                      | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAcfLinPart`                                  | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpAvConnection`                                | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpConfig`                                      | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpConnection`                                  | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpCrfConnection`                               | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpCrfPullEnum`                                 | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpCrfTypeEnum`                                 | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpIidcConnection`                              | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpRvfColorSpaceEnum`                           | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpRvfConnection`                               | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpRvfFrameRateEnum`                            | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpRvfPixelDepthEnum`                           | [ ] Created | N/A                                      | Group33          |
| `IEEE1722TpRvfPixelFormatEnum`                          | [ ] Created | N/A                                      | Group33          |
| `IPSecConfig`                                           | [ ] Implemented| N/A                                      | Group16          |
| `IPSecConfigProps`                                      | [ ] Implemented| N/A                                      | Group32          |
| `IPSecRule`                                             | [ ] Implemented| N/A                                      | Group32          |
| `IPdu`                                                  | [ ] Implemented| N/A                                      | Group31          |
| `IPduMapping`                                           | [ ] Implemented| N/A                                      | Group6           |
| `IPduPort`                                              | [ ] Implemented| N/A                                      | Group31          |
| `IPduSignalProcessingEnum`                              | [ ] Implemented| N/A                                      | Group31          |
| `IPduTiming`                                            | [ ] Implemented| N/A                                      | Group31          |
| `IPsecDpdActionEnum`                                    | [ ] Implemented| N/A                                      | Group33          |
| `IPsecHeaderTypeEnum`                                   | [ ] Implemented| N/A                                      | Group33          |
| `IPsecIpProtocolEnum`                                   | [ ] Implemented| N/A                                      | Group32          |
| `IPsecModeEnum`                                         | [ ] Implemented| N/A                                      | Group32          |
| `IPsecPolicyEnum`                                       | [ ] Implemented| N/A                                      | Group32          |
| `IPv6ExtHeaderFilterList`                               | [ ] Implemented| N/A                                      | Group16          |
| `IPv6ExtHeaderFilterSet`                                | [ ] Created | N/A                                      | Group32          |
| `ISignal`                                               | [ ] Implemented| N/A                                      | Group31          |
| `ISignalGroup`                                          | [ ] Implemented| N/A                                      | Group31          |
| `ISignalIPdu`                                           | [ ] Implemented| N/A                                      | Group31          |
| `ISignalIPduGroup`                                      | [ ] Implemented| N/A                                      | Group15          |
| `ISignalMapping`                                        | [ ] Implemented| N/A                                      | Group17          |
| `ISignalPort`                                           | [ ] Implemented| N/A                                      | Group15          |
| `ISignalProps`                                          | [ ] Implemented| N/A                                      | Group31          |
| `ISignalToIPduMapping`                                  | [ ] Implemented| N/A                                      | Group31          |
| `ISignalTriggering`                                     | [ ] Implemented| N/A                                      | Group31          |
| `ISignalTypeEnum`                                       | [ ] Implemented| N/A                                      | Group31          |
| `IcmpRule`                                              | [ ] Implemented| N/A                                      | Group20          |
| `IdentCaption`                                          | [ ] Implemented| N/A                                      | Group1           |
| `Identifiable`                                          | [ ] Implemented| N/A                                      | Group1           |
| `Identifier`                                            | [ ] Implemented| N/A                                      | Group21          |
| `IdsDesign`                                             | [ ] Created | N/A                                      | Group36          |
| `IdsMgrCustomTimestampNeeds`                            | [ ] Implemented| N/A                                      | Group5           |
| `IdsMgrNeeds`                                           | [ ] Implemented| N/A                                      | Group29          |
| `IdsPlatformInstantiation`                              | [ ] Implemented| N/A                                      | Group7           |
| `IdsmInstance`                                          | [ ] Created | N/A                                      | Group36          |
| `IdsmModuleInstantiation`                               | [ ] Implemented| N/A                                      | Group7           |
| `IdsmRateLimitation`                                    | [ ] Created | N/A                                      | Group36          |
| `IdsmTrafficLimitation`                                 | [ ] Created | N/A                                      | Group36          |
| `Ieee1722Tp`                                            | [ ] Created | N/A                                      | Group32          |
| `Ieee1722TpEthernetFrame`                               | [ ] Created | N/A                                      | Group33          |
| `Implementation`                                        | [ ] Implemented| N/A                                      | Group1           |
| `ImplementationDataType`                                | [ ] Implemented| N/A                                      | Group28          |
| `ImplementationDataTypeElement`                         | [ ] Implemented| N/A                                      | Group10          |
| `ImplementationDataTypeElementInPortInterfaceRef`       | [ ] Implemented| N/A                                      | Group34          |
| `ImplementationDataTypeSubElementRef`                   | [ ] Created | N/A                                      | Group27          |
| `ImplementationElementInParameterInstanceRef`           | [ ] Implemented| N/A                                      | Group23          |
| `ImplementationProps`                                   | [ ] Implemented| N/A                                      | Group10          |
| `IncludedDataTypeSet`                                   | [ ] Implemented| N/A                                      | Group29          |
| `IncludedModeDeclarationGroupSet`                       | [ ] Implemented| N/A                                      | Group2           |
| `IndentSample`                                          | [ ] Implemented| N/A                                      | Group21          |
| `IndexEntry`                                            | [ ] Implemented| N/A                                      | Group21          |
| `IndexedArrayElement`                                   | [ ] Implemented| N/A                                      | Group17          |
| `IndicatorStatusNeeds`                                  | [ ] Implemented| N/A                                      | Group29          |
| `InfrastructureServices`                                | [ ] Implemented| N/A                                      | Group32          |
| `InitEvent`                                             | [ ] Implemented| N/A                                      | Group2           |
| `InitialSdDelayConfig`                                  | [ ] Implemented| N/A                                      | Group16          |
| `InnerPortGroupInCompositionInstanceRef`                | [ ] Implemented| N/A                                      | Group2           |
| `InstantiationDataDefProps`                             | [ ] Implemented| N/A                                      | Group10          |
| `InstantiationRTEEventProps`                            | [ ] Implemented| N/A                                      | Group27          |
| `InstantiationTimingEventProps`                         | [ ] Implemented| N/A                                      | Group27          |
| `IntegerValueVariationPoint`                            | [ ] Implemented| N/A                                      | Group8           |
| `InternalBehavior`                                      | [ ] Implemented| N/A                                      | Group22          |
| `InternalConstrs`                                       | [ ] Implemented| N/A                                      | Group28          |
| `InternalTriggerOccurredEvent`                          | [ ] Implemented| N/A                                      | Group12          |
| `InternalTriggeringPoint`                               | [ ] Implemented| N/A                                      | Group12          |
| `InterpolationRoutine`                                  | [ ] Implemented| N/A                                      | Group5           |
| `InterpolationRoutineMapping`                           | [ ] Implemented| N/A                                      | Group5           |
| `InterpolationRoutineMappingSet`                        | [ ] Implemented| N/A                                      | Group5           |
| `IntervalTypeEnum`                                      | [ ] Implemented| N/A                                      | Group28          |
| `InvalidationPolicy`                                    | [ ] Implemented| N/A                                      | Group1           |
| `InvertCondition`                                       | [ ] Created | N/A                                      | Group36          |
| `IoHwAbstractionServerAnnotation`                       | [ ] Implemented| N/A                                      | Group27          |
| `Ip4AddressString`                                      | [ ] Implemented| N/A                                      | Group21          |
| `Ip6AddressString`                                      | [ ] Implemented| N/A                                      | Group21          |
| `IpAddressKeepEnum`                                     | [ ] Implemented| N/A                                      | Group16          |
| `Ipv4AddressSourceEnum`                                 | [ ] Implemented| N/A                                      | Group16          |
| `Ipv4ArpProps`                                          | [ ] Created | N/A                                      | Group30          |
| `Ipv4AutoIpProps`                                       | [ ] Created | N/A                                      | Group30          |
| `Ipv4Configuration`                                     | [ ] Implemented| N/A                                      | Group16          |
| `Ipv4DhcpServerConfiguration`                           | [ ] Implemented| N/A                                      | Group30          |
| `Ipv4FragmentationProps`                                | [ ] Created | N/A                                      | Group30          |
| `Ipv4Props`                                             | [ ] Created | N/A                                      | Group30          |
| `Ipv4Rule`                                              | [ ] Implemented| N/A                                      | Group20          |
| `Ipv6AddressSourceEnum`                                 | [ ] Implemented| N/A                                      | Group16          |
| `Ipv6Configuration`                                     | [ ] Implemented| N/A                                      | Group32          |
| `Ipv6DhcpServerConfiguration`                           | [ ] Implemented| N/A                                      | Group30          |
| `Ipv6FragmentationProps`                                | [ ] Created | N/A                                      | Group30          |
| `Ipv6NdpProps`                                          | [ ] Created | N/A                                      | Group30          |
| `Ipv6Props`                                             | [ ] Created | N/A                                      | Group30          |
| `Ipv6Rule`                                              | [ ] Implemented| N/A                                      | Group20          |
| `Item`                                                  | [ ] Implemented| N/A                                      | Group9           |
| `ItemLabelPosEnum`                                      | [ ] Implemented| N/A                                      | Group21          |
| `J1939Cluster`                                          | [ ] Implemented| N/A                                      | Group5           |
| `J1939ControllerApplication`                            | [ ] Created | N/A                                      | Group30          |
| `J1939ControllerApplicationToJ1939NmNodeMapping`        | [ ] Created | N/A                                      | Group30          |
| `J1939DcmDm19Support`                                   | [ ] Implemented| N/A                                      | Group5           |
| `J1939DcmIPdu`                                          | [ ] Created | N/A                                      | Group31          |
| `J1939NmAddressConfigurationCapabilityEnum`             | [ ] Implemented| N/A                                      | Group33          |
| `J1939NmCluster`                                        | [ ] Implemented| N/A                                      | Group6           |
| `J1939NmEcu`                                            | [ ] Implemented| N/A                                      | Group6           |
| `J1939NmNode`                                           | [ ] Implemented| N/A                                      | Group33          |
| `J1939NodeName`                                         | [ ] Implemented| N/A                                      | Group33          |
| `J1939RmIncomingRequestServiceNeeds`                    | [ ] Implemented| N/A                                      | Group5           |
| `J1939RmOutgoingRequestServiceNeeds`                    | [ ] Implemented| N/A                                      | Group5           |
| `J1939SharedAddressCluster`                             | [ ] Implemented| N/A                                      | Group5           |
| `J1939TpConfig`                                         | [ ] Created | N/A                                      | Group33          |
| `J1939TpConnection`                                     | [ ] Created | N/A                                      | Group33          |
| `J1939TpNode`                                           | [ ] Created | N/A                                      | Group33          |
| `J1939TpPg`                                             | [ ] Created | N/A                                      | Group33          |
| `KeepWithPreviousEnum`                                  | [ ] Implemented| N/A                                      | Group3           |
| `Keyword`                                               | [ ] Implemented| N/A                                      | Group7           |
| `KeywordSet`                                            | [ ] Implemented| N/A                                      | Group7           |
| `LEnum`                                                 | [ ] Implemented| N/A                                      | Group22          |
| `LGraphic`                                              | [ ] Implemented| N/A                                      | Group3           |
| `LLongName`                                             | [ ] Implemented| N/A                                      | Group21          |
| `LOverviewParagraph`                                    | [ ] Implemented| N/A                                      | Group9           |
| `LParagraph`                                            | [ ] Implemented| N/A                                      | Group3           |
| `LPlainText`                                            | [ ] Implemented| N/A                                      | Group9           |
| `LVerbatim`                                             | [ ] Implemented| N/A                                      | Group9           |
| `LabeledItem`                                           | [ ] Implemented| N/A                                      | Group21          |
| `LabeledList`                                           | [ ] Implemented| N/A                                      | Group21          |
| `LanguageSpecific`                                      | [ ] Implemented| N/A                                      | Group22          |
| `LatencyConstraintTypeEnum`                             | [ ] Implemented| N/A                                      | Group35          |
| `LatencyTimingConstraint`                               | [ ] Implemented| N/A                                      | Group35          |
| `LetDataExchangeParadigmEnum`                           | [ ] Implemented| N/A                                      | Group35          |
| `LifeCycleInfo`                                         | [ ] Implemented| N/A                                      | Group8           |
| `LifeCycleInfoSet`                                      | [ ] Implemented| N/A                                      | Group8           |
| `LifeCyclePeriod`                                       | [ ] Implemented| N/A                                      | Group8           |
| `LifeCycleState`                                        | [ ] Created | N/A                                      | Group22          |
| `LifeCycleStateDefinitionGroup`                         | [ ] Implemented| N/A                                      | Group22          |
| `Limit`                                                 | [ ] Implemented| N/A                                      | Group28          |
| `LimitValueVariationPoint`                              | [ ] Implemented| N/A                                      | Group8           |
| `LinChecksumType`                                       | [ ] Created | N/A                                      | Group31          |
| `LinCluster`                                            | [ ] Implemented| N/A                                      | Group29          |
| `LinCommunicationConnector`                             | [ ] Implemented| N/A                                      | Group17          |
| `LinCommunicationController`                            | [ ] Implemented| N/A                                      | Group29          |
| `LinConfigurableFrame`                                  | [ ] Implemented| N/A                                      | Group29          |
| `LinConfigurationEntry`                                 | [ ] Implemented| N/A                                      | Group31          |
| `LinErrorResponse`                                      | [ ] Implemented| N/A                                      | Group29          |
| `LinEventTriggeredFrame`                                | [ ] Created | N/A                                      | Group31          |
| `LinFrame`                                              | [ ] Implemented| N/A                                      | Group31          |
| `LinFrameTriggering`                                    | [ ] Implemented| N/A                                      | Group31          |
| `LinMaster`                                             | [ ] Implemented| N/A                                      | Group29          |
| `LinOrderedConfigurableFrame`                           | [ ] Implemented| N/A                                      | Group29          |
| `LinPhysicalChannel`                                    | [ ] Implemented| N/A                                      | Group29          |
| `LinScheduleTable`                                      | [ ] Implemented| N/A                                      | Group17          |
| `LinSlave`                                              | [ ] Created | N/A                                      | Group29          |
| `LinSlaveConfig`                                        | [ ] Implemented| N/A                                      | Group29          |
| `LinSlaveConfigIdent`                                   | [ ] Implemented| N/A                                      | Group29          |
| `LinSporadicFrame`                                      | [ ] Created | N/A                                      | Group31          |
| `LinTpConfig`                                           | [ ] Implemented| N/A                                      | Group33          |
| `LinTpConnection`                                       | [ ] Implemented| N/A                                      | Group18          |
| `LinTpNode`                                             | [ ] Implemented| N/A                                      | Group33          |
| `LinUnconditionalFrame`                                 | [ ] Implemented| N/A                                      | Group31          |
| `Linker`                                                | [ ] Implemented| N/A                                      | Group1           |
| `List`                                                  | [ ] Pending | N/A                                      | Group21          |
| `ListEnum`                                              | [ ] Implemented| N/A                                      | Group9           |
| `LogAndTraceMessageCollectionSet`                       | [ ] Created | N/A                                      | Group36          |
| `LogTraceDefaultLogLevelEnum`                           | [ ] Implemented| N/A                                      | Group5           |
| `MacAddressString`                                      | [ ] Implemented| N/A                                      | Group20          |
| `MacMulticastConfiguration`                             | [ ] Created | N/A                                      | Group32          |
| `MacMulticastGroup`                                     | [ ] Implemented| N/A                                      | Group16          |
| `MacSecCapabilityEnum`                                  | [ ] Implemented| N/A                                      | Group30          |
| `MacSecCipherSuiteConfig`                               | [ ] Implemented| N/A                                      | Group30          |
| `MacSecConfidentialityOffsetEnum`                       | [ ] Implemented| N/A                                      | Group30          |
| `MacSecCryptoAlgoConfig`                                | [ ] Implemented| N/A                                      | Group30          |
| `MacSecFailPermissiveModeEnum`                          | [ ] Implemented| N/A                                      | Group30          |
| `MacSecGlobalKayProps`                                  | [ ] Implemented| N/A                                      | Group30          |
| `MacSecKayParticipant`                                  | [ ] Implemented| N/A                                      | Group30          |
| `MacSecLocalKayProps`                                   | [ ] Implemented| N/A                                      | Group30          |
| `MacSecParticipantSet`                                  | [ ] Created | N/A                                      | Group30          |
| `MacSecProps`                                           | [ ] Implemented| N/A                                      | Group30          |
| `MacSecRoleEnum`                                        | [ ] Implemented| N/A                                      | Group30          |
| `Map`                                                   | [ ] Implemented| N/A                                      | Group3           |
| `MappingConstraint`                                     | [ ] Created | N/A                                      | Group30          |
| `MappingDirectionEnum`                                  | [ ] Implemented| N/A                                      | Group27          |
| `MappingScopeEnum`                                      | [ ] Created | N/A                                      | Group30          |
| `MaxCommModeEnum`                                       | [ ] Implemented| N/A                                      | Group23          |
| `MaximumMessageLengthType`                              | [ ] Created | N/A                                      | Group33          |
| `McDataAccessDetails`                                   | [ ] Implemented| N/A                                      | Group23          |
| `McDataInstance`                                        | [ ] Implemented| N/A                                      | Group23          |
| `McFunction`                                            | [ ] Implemented| N/A                                      | Group23          |
| `McFunctionDataRefSet`                                  | [ ] Implemented| N/A                                      | Group23          |
| `McGroup`                                               | [ ] Implemented| N/A                                      | Group23          |
| `McGroupDataRefSet`                                     | [ ] Implemented| N/A                                      | Group23          |
| `McParameterElementGroup`                               | [ ] Implemented| N/A                                      | Group23          |
| `McSupportData`                                         | [ ] Implemented| N/A                                      | Group23          |
| `McSwEmulationMethodSupport`                            | [ ] Implemented| N/A                                      | Group23          |
| `McdIdentifier`                                         | [ ] Implemented| N/A                                      | Group21          |
| `MeasuredExecutionTime`                                 | [ ] Implemented| N/A                                      | Group23          |
| `MeasuredHeapUsage`                                     | [ ] Implemented| N/A                                      | Group23          |
| `MeasuredStackUsage`                                    | [ ] Implemented| N/A                                      | Group20          |
| `MemoryAllocationKeywordPolicyType`                     | [ ] Implemented| N/A                                      | Group22          |
| `MemorySection`                                         | [ ] Implemented| N/A                                      | Group20          |
| `MemorySectionLocation`                                 | [ ] Implemented| N/A                                      | Group23          |
| `MemorySectionType`                                     | [ ] Implemented| N/A                                      | Group22          |
| `MetaDataItem`                                          | [ ] Implemented| N/A                                      | Group2           |
| `MetaDataItemSet`                                       | [ ] Implemented| N/A                                      | Group2           |
| `MimeTypeString`                                        | [ ] Implemented| N/A                                      | Group3           |
| `MirroringProtocolEnum`                                 | [ ] Created | N/A                                      | Group33          |
| `MixedContentForLongName`                               | [ ] Implemented| N/A                                      | Group21          |
| `MixedContentForOverviewParagraph`                      | [ ] Implemented| N/A                                      | Group8           |
| `MixedContentForParagraph`                              | [ ] Implemented| N/A                                      | Group3           |
| `MixedContentForPlainText`                              | [ ] Implemented| N/A                                      | Group8           |
| `MixedContentForUnitNames`                              | [ ] Implemented| N/A                                      | Group8           |
| `MixedContentForVerbatim`                               | [ ] Implemented| N/A                                      | Group8           |
| `MlFigure`                                              | [ ] Implemented| N/A                                      | Group3           |
| `MlFormula`                                             | [ ] Implemented| N/A                                      | Group21          |
| `ModeAccessPoint`                                       | [ ] Implemented| N/A                                      | Group12          |
| `ModeAccessPointIdent`                                  | [ ] Implemented| N/A                                      | Group1           |
| `ModeActivationKind`                                    | [ ] Implemented| N/A                                      | Group11          |
| `ModeDeclaration`                                       | [ ] Implemented| N/A                                      | Group22          |
| `ModeDeclarationGroup`                                  | [ ] Implemented| N/A                                      | Group22          |
| `ModeDeclarationGroupPrototype`                         | [ ] Implemented| N/A                                      | Group1           |
| `ModeDeclarationGroupPrototypeMapping`                  | [ ] Implemented| N/A                                      | Group11          |
| `ModeDeclarationMapping`                                | [ ] Implemented| N/A                                      | Group27          |
| `ModeDeclarationMappingSet`                             | [ ] Implemented| N/A                                      | Group1           |
| `ModeDrivenTransmissionModeCondition`                   | [ ] Implemented| N/A                                      | Group5           |
| `ModeErrorBehavior`                                     | [ ] Implemented| N/A                                      | Group22          |
| `ModeErrorReactionPolicyEnum`                           | [ ] Implemented| N/A                                      | Group22          |
| `ModeGroupInAtomicSwcInstanceRef`                       | [ ] Implemented| N/A                                      | Group11          |
| `ModeInBswInstanceRef`                                  | [ ] Implemented| N/A                                      | Group35          |
| `ModeInSwcBswInstanceRef`                               | [ ] Implemented| N/A                                      | Group8           |
| `ModeInSwcInstanceRef`                                  | [ ] Implemented| N/A                                      | Group8           |
| `ModeInterfaceMapping`                                  | [ ] Implemented| N/A                                      | Group11          |
| `ModePortAnnotation`                                    | [ ] Implemented| N/A                                      | Group27          |
| `ModeRequestTypeMap`                                    | [ ] Implemented| N/A                                      | Group11          |
| `ModeSwitchEventTriggeredActivity`                      | [ ] Implemented| N/A                                      | Group10          |
| `ModeSwitchInterface`                                   | [ ] Implemented| N/A                                      | Group27          |
| `ModeSwitchPoint`                                       | [ ] Implemented| N/A                                      | Group12          |
| `ModeSwitchReceiverComSpec`                             | [ ] Implemented| N/A                                      | Group10          |
| `ModeSwitchSenderComSpec`                               | [ ] Implemented| N/A                                      | Group10          |
| `ModeSwitchedAckEvent`                                  | [ ] Implemented| N/A                                      | Group28          |
| `ModeSwitchedAckRequest`                                | [ ] Implemented| N/A                                      | Group10          |
| `ModeTransition`                                        | [ ] Implemented| N/A                                      | Group22          |
| `Modification`                                          | [ ] Implemented| N/A                                      | Group9           |
| `ModuleConfiguration`                                   | [ ] Implemented| N/A                                      | Group19          |
| `MonotonyEnum`                                          | [ ] Implemented| N/A                                      | Group28          |
| `MsrQueryArg`                                           | [ ] Implemented| N/A                                      | Group22          |
| `MsrQueryChapter`                                       | [ ] Implemented| N/A                                      | Group3           |
| `MsrQueryP1`                                            | [ ] Implemented| N/A                                      | Group3           |
| `MsrQueryProps`                                         | [ ] Implemented| N/A                                      | Group22          |
| `MsrQueryResultChapter`                                 | [ ] Implemented| N/A                                      | Group3           |
| `MsrQueryResultTopic1`                                  | [ ] Implemented| N/A                                      | Group3           |
| `MsrQueryTopic1`                                        | [ ] Implemented| N/A                                      | Group3           |
| `MultiLanguageOverviewParagraph`                        | [ ] Implemented| N/A                                      | Group22          |
| `MultiLanguageParagraph`                                | [ ] Implemented| N/A                                      | Group3           |
| `MultiLanguagePlainText`                                | [ ] Implemented| N/A                                      | Group22          |
| `MultiLanguageVerbatim`                                 | [ ] Implemented| N/A                                      | Group21          |
| `MultidimensionalTime`                                  | [ ] Implemented| N/A                                      | Group8           |
| `MultilanguageLongName`                                 | [ ] Implemented| N/A                                      | Group3           |
| `MultilanguageReferrable`                               | [ ] Implemented| N/A                                      | Group1           |
| `MultiplexedIPdu`                                       | [ ] Implemented| N/A                                      | Group15          |
| `MultiplexedPart`                                       | [ ] Implemented| N/A                                      | Group15          |
| `MultiplicityRestrictionWithSeverity`                   | [ ] Created | N/A                                      | Group36          |
| `NPdu`                                                  | [ ] Implemented| N/A                                      | Group31          |
| `NameTokens`                                            | [ ] Implemented| N/A                                      | Group3           |
| `NetworkEndpoint`                                       | [ ] Implemented| N/A                                      | Group16          |
| `NetworkEndpointAddress`                                | [ ] Implemented| N/A                                      | Group6           |
| `NetworkLayerRule`                                      | [ ] Implemented| N/A                                      | Group20          |
| `NetworkSegmentIdentification`                          | [ ] Created | N/A                                      | Group34          |
| `NetworkTargetAddressType`                              | [ ] Implemented| N/A                                      | Group33          |
| `NmCluster`                                             | [ ] Implemented| N/A                                      | Group6           |
| `NmClusterCoupling`                                     | [ ] Implemented| N/A                                      | Group6           |
| `NmConfig`                                              | [ ] Implemented| N/A                                      | Group6           |
| `NmCoordinator`                                         | [ ] Created | N/A                                      | Group33          |
| `NmCoordinatorRoleEnum`                                 | [ ] Implemented| N/A                                      | Group33          |
| `NmEcu`                                                 | [ ] Implemented| N/A                                      | Group18          |
| `NmNode`                                                | [ ] Implemented| N/A                                      | Group33          |
| `NmPdu`                                                 | [ ] Implemented| N/A                                      | Group31          |
| `NonqueuedReceiverComSpec`                              | [ ] Implemented| N/A                                      | Group27          |
| `NonqueuedSenderComSpec`                                | [ ] Implemented| N/A                                      | Group27          |
| `NotAvailableValueSpecification`                        | [ ] Implemented| N/A                                      | Group28          |
| `Note`                                                  | [ ] Implemented| N/A                                      | Group21          |
| `NoteTypeEnum`                                          | [ ] Implemented| N/A                                      | Group21          |
| `NumericalOrText`                                       | [ ] Implemented| N/A                                      | Group28          |
| `NumericalRuleBasedValueSpecification`                  | [ ] Implemented| N/A                                      | Group28          |
| `NumericalValueSpecification`                           | [ ] Implemented| N/A                                      | Group9           |
| `NumericalValueVariationPoint`                          | [ ] Implemented| N/A                                      | Group8           |
| `NvBlockDataMapping`                                    | [ ] Implemented| N/A                                      | Group10          |
| `NvBlockDescriptor`                                     | [ ] Implemented| N/A                                      | Group10          |
| `NvBlockNeeds`                                          | [ ] Implemented| N/A                                      | Group10          |
| `NvBlockNeedsReliabilityEnum`                           | [ ] Implemented| N/A                                      | Group10          |
| `NvBlockNeedsWritingPriorityEnum`                       | [ ] Implemented| N/A                                      | Group10          |
| `NvBlockSwComponentType`                                | [ ] Implemented| N/A                                      | Group29          |
| `NvDataInterface`                                       | [ ] Implemented| N/A                                      | Group1           |
| `NvDataPortAnnotation`                                  | [ ] Implemented| N/A                                      | Group27          |
| `NvProvideComSpec`                                      | [ ] Implemented| N/A                                      | Group10          |
| `NvRequireComSpec`                                      | [ ] Implemented| N/A                                      | Group10          |
| `ObdControlServiceNeeds`                                | [ ] Implemented| N/A                                      | Group29          |
| `ObdInfoServiceNeeds`                                   | [ ] Implemented| N/A                                      | Group29          |
| `ObdMonitorServiceNeeds`                                | [ ] Implemented| N/A                                      | Group29          |
| `ObdPidServiceNeeds`                                    | [ ] Implemented| N/A                                      | Group29          |
| `ObdRatioConnectionKindEnum`                            | [ ] Implemented| N/A                                      | Group29          |
| `ObdRatioDenominatorNeeds`                              | [ ] Implemented| N/A                                      | Group29          |
| `ObdRatioServiceNeeds`                                  | [ ] Implemented| N/A                                      | Group29          |
| `OffsetTimingConstraint`                                | [ ] Implemented| N/A                                      | Group8           |
| `OperationCycleTypeEnum`                                | [ ] Implemented| N/A                                      | Group29          |
| `OperationInAtomicSwcInstanceRef`                       | [ ] Implemented| N/A                                      | Group11          |
| `OperationInSystemInstanceRef`                          | [ ] Implemented| N/A                                      | Group5           |
| `OperationInvokedEvent`                                 | [ ] Implemented| N/A                                      | Group12          |
| `OrderedMaster`                                         | [ ] Implemented| N/A                                      | Group6           |
| `OrientEnum`                                            | [ ] Implemented| N/A                                      | Group3           |
| `OsTaskExecutionEvent`                                  | [ ] Created | N/A                                      | Group28          |
| `OsTaskPreemptabilityEnum`                              | [ ] Implemented| N/A                                      | Group5           |
| `OsTaskProxy`                                           | [ ] Implemented| N/A                                      | Group5           |
| `PModeGroupInAtomicSwcInstanceRef`                      | [ ] Implemented| N/A                                      | Group11          |
| `POperationInAtomicSwcInstanceRef`                      | [ ] Implemented| N/A                                      | Group11          |
| `PPortComSpec`                                          | [ ] Implemented| N/A                                      | Group27          |
| `PPortInCompositionInstanceRef`                         | [ ] Implemented| N/A                                      | Group11          |
| `PPortPrototype`                                        | [ ] Implemented| N/A                                      | Group2           |
| `PRPortPrototype`                                       | [ ] Implemented| N/A                                      | Group2           |
| `PTriggerInAtomicSwcTypeInstanceRef`                    | [ ] Implemented| N/A                                      | Group11          |
| `PackageableElement`                                    | [ ] Implemented| N/A                                      | Group1           |
| `Paginateable`                                          | [ ] Implemented| N/A                                      | Group3           |
| `ParameterAccess`                                       | [ ] Implemented| N/A                                      | Group12          |
| `ParameterDataPrototype`                                | [ ] Implemented| N/A                                      | Group22          |
| `ParameterInAtomicSWCTypeInstanceRef`                   | [ ] Implemented| N/A                                      | Group28          |
| `ParameterInterface`                                    | [ ] Implemented| N/A                                      | Group1           |
| `ParameterPortAnnotation`                               | [ ] Implemented| N/A                                      | Group27          |
| `ParameterProvideComSpec`                               | [ ] Implemented| N/A                                      | Group27          |
| `ParameterRequireComSpec`                               | [ ] Implemented| N/A                                      | Group10          |
| `ParameterSwComponentType`                              | [ ] Created | N/A                                      | Group27          |
| `PassThroughSwConnector`                                | [ ] Implemented| N/A                                      | Group27          |
| `PayloadBytePatternRule`                                | [ ] Implemented| N/A                                      | Group20          |
| `PayloadBytePatternRulePart`                            | [ ] Implemented| N/A                                      | Group20          |
| `Pdu`                                                   | [ ] Implemented| N/A                                      | Group31          |
| `PduActivationRoutingGroup`                             | [ ] Implemented| N/A                                      | Group32          |
| `PduCollectionSemanticsEnum`                            | [ ] Implemented| N/A                                      | Group16          |
| `PduCollectionTriggerEnum`                              | [ ] Implemented| N/A                                      | Group5           |
| `PduMappingDefaultValue`                                | [ ] Implemented| N/A                                      | Group6           |
| `PduToFrameMapping`                                     | [ ] Implemented| N/A                                      | Group31          |
| `PduTriggering`                                         | [ ] Implemented| N/A                                      | Group31          |
| `PdurIPduGroup`                                         | [ ] Implemented| N/A                                      | Group5           |
| `PerInstanceMemory`                                     | [ ] Implemented| N/A                                      | Group2           |
| `PerInstanceMemorySize`                                 | [ ] Implemented| N/A                                      | Group10          |
| `PeriodicEventTriggering`                               | [ ] Implemented| N/A                                      | Group35          |
| `PermissibleSignalPath`                                 | [ ] Created | N/A                                      | Group31          |
| `PgwideEnum`                                            | [ ] Implemented| N/A                                      | Group22          |
| `PhysConstrs`                                           | [ ] Implemented| N/A                                      | Group28          |
| `PhysicalChannel`                                       | [ ] Implemented| N/A                                      | Group29          |
| `PhysicalDimension`                                     | [ ] Implemented| N/A                                      | Group28          |
| `PhysicalDimensionMapping`                              | [ ] Created | N/A                                      | Group28          |
| `PhysicalDimensionMappingSet`                           | [ ] Created | N/A                                      | Group28          |
| `PlatformModuleEthernetEndpointConfiguration`           | [ ] Implemented| N/A                                      | Group7           |
| `PlcaProps`                                             | [ ] Implemented| N/A                                      | Group30          |
| `PncGatewayTypeEnum`                                    | [ ] Implemented| N/A                                      | Group15          |
| `PncMapping`                                            | [ ] Created | N/A                                      | Group31          |
| `PortAPIOption`                                         | [ ] Implemented| N/A                                      | Group2           |
| `PortDefinedArgumentValue`                              | [ ] Implemented| N/A                                      | Group2           |
| `PortElementToCommunicationResourceMapping`             | [ ] Created | N/A                                      | Group34          |
| `PortGroup`                                             | [ ] Implemented| N/A                                      | Group2           |
| `PortGroupInSystemInstanceRef`                          | [ ] Implemented| N/A                                      | Group5           |
| `PortInCompositionTypeInstanceRef`                      | [ ] Implemented| N/A                                      | Group2           |
| `PortInterface`                                         | [ ] Implemented| N/A                                      | Group27          |
| `PortInterfaceBlueprintMapping`                         | [ ] Implemented| N/A                                      | Group1           |
| `PortInterfaceMapping`                                  | [ ] Implemented| N/A                                      | Group1           |
| `PortInterfaceMappingSet`                               | [ ] Implemented| N/A                                      | Group2           |
| `PortPrototype`                                         | [ ] Implemented| N/A                                      | Group1           |
| `PortPrototypeBlueprint`                                | [ ] Implemented| N/A                                      | Group7           |
| `PortPrototypeBlueprintInitValue`                       | [ ] Implemented| N/A                                      | Group7           |
| `PortPrototypeBlueprintMapping`                         | [ ] Implemented| N/A                                      | Group1           |
| `PositiveIntegerValueVariationPoint`                    | [ ] Implemented| N/A                                      | Group8           |
| `PostBuildVariantCondition`                             | [ ] Implemented| N/A                                      | Group8           |
| `PostBuildVariantCriterion`                             | [ ] Implemented| N/A                                      | Group8           |
| `PostBuildVariantCriterionValue`                        | [ ] Implemented| N/A                                      | Group8           |
| `PostBuildVariantCriterionValueSet`                     | [ ] Created | N/A                                      | Group36          |
| `PredefinedChapter`                                     | [ ] Implemented| N/A                                      | Group22          |
| `PredefinedVariant`                                     | [ ] Implemented| N/A                                      | Group21          |
| `PrimitiveAttributeCondition`                           | [ ] Created | N/A                                      | Group36          |
| `PrimitiveAttributeTailoring`                           | [ ] Created | N/A                                      | Group36          |
| `PrivacyLevel`                                          | [ ] Implemented| N/A                                      | Group5           |
| `PrmChar`                                               | [ ] Implemented| N/A                                      | Group9           |
| `PrmCharAbsTol`                                         | [ ] Implemented| N/A                                      | Group9           |
| `PrmCharContents`                                       | [ ] Implemented| N/A                                      | Group9           |
| `PrmCharMinTypMax`                                      | [ ] Implemented| N/A                                      | Group9           |
| `PrmCharNumericalContents`                              | [ ] Implemented| N/A                                      | Group9           |
| `PrmCharNumericalValue`                                 | [ ] Implemented| N/A                                      | Group9           |
| `PrmCharTextualContents`                                | [ ] Implemented| N/A                                      | Group9           |
| `Prms`                                                  | [ ] Implemented| N/A                                      | Group9           |
| `ProcessingKindEnum`                                    | [ ] Implemented| N/A                                      | Group27          |
| `ProgramminglanguageEnum`                               | [ ] Implemented| N/A                                      | Group1           |
| `ProvidedServiceInstance`                               | [ ] Implemented| N/A                                      | Group32          |
| `PulseTestEnum`                                         | [ ] Implemented| N/A                                      | Group27          |
| `QueuedReceiverComSpec`                                 | [ ] Implemented| N/A                                      | Group10          |
| `QueuedSenderComSpec`                                   | [ ] Implemented| N/A                                      | Group5           |
| `RModeGroupInAtomicSWCInstanceRef`                      | [ ] Implemented| N/A                                      | Group11          |
| `RModeInAtomicSwcInstanceRef`                           | [ ] Implemented| N/A                                      | Group11          |
| `ROperationInAtomicSwcInstanceRef`                      | [ ] Implemented| N/A                                      | Group11          |
| `RPortComSpec`                                          | [ ] Implemented| N/A                                      | Group27          |
| `RPortInCompositionInstanceRef`                         | [ ] Implemented| N/A                                      | Group11          |
| `RPortPrototype`                                        | [ ] Implemented| N/A                                      | Group2           |
| `RTEEvent`                                              | [ ] Implemented| N/A                                      | Group2           |
| `RVariableInAtomicSwcInstanceRef`                       | [ ] Implemented| N/A                                      | Group11          |
| `RamBlockStatusControlEnum`                             | [ ] Implemented| N/A                                      | Group10          |
| `RapidPrototypingScenario`                              | [ ] Created | N/A                                      | Group29          |
| `ReceiverAnnotation`                                    | [ ] Created | N/A                                      | Group27          |
| `ReceiverComSpec`                                       | [ ] Implemented| N/A                                      | Group27          |
| `ReceptionComSpecProps`                                 | [ ] Implemented| N/A                                      | Group10          |
| `RecordLayoutIteratorPoint`                             | [ ] Created | N/A                                      | Group3           |
| `RecordValueSpecification`                              | [ ] Implemented| N/A                                      | Group3           |
| `ReentrancyLevelEnum`                                   | [ ] Implemented| N/A                                      | Group10          |
| `Ref`                                                   | [ ] Implemented| N/A                                      | Group3           |
| `ReferenceBase`                                         | [ ] Implemented| N/A                                      | Group1           |
| `ReferenceCondition`                                    | [ ] Created | N/A                                      | Group36          |
| `ReferenceTailoring`                                    | [ ] Created | N/A                                      | Group36          |
| `ReferenceValueSpecification`                           | [ ] Implemented| N/A                                      | Group28          |
| `Referrable`                                            | [ ] Implemented| N/A                                      | Group21          |
| `ReferrableSubtypesEnum`                                | [ ] Implemented| N/A                                      | Group21          |
| `RegularExpression`                                     | [ ] Implemented| N/A                                      | Group21          |
| `RelativeTolerance`                                     | [ ] Created | N/A                                      | Group31          |
| `RequestResponseDelay`                                  | [ ] Implemented| N/A                                      | Group16          |
| `ResolutionPolicyEnum`                                  | [ ] Implemented| N/A                                      | Group3           |
| `ResourceConsumption`                                   | [ ] Implemented| N/A                                      | Group1           |
| `RestrictionWithSeverity`                               | [ ] Created | N/A                                      | Group36          |
| `ResumePosition`                                        | [ ] Implemented| N/A                                      | Group17          |
| `RevisionLabelString`                                   | [ ] Created | N/A                                      | Group21          |
| `RoleBasedBswModuleEntryAssignment`                     | [ ] Implemented| N/A                                      | Group23          |
| `RoleBasedDataAssignment`                               | [ ] Implemented| N/A                                      | Group10          |
| `RoleBasedDataTypeAssignment`                           | [ ] Implemented| N/A                                      | Group23          |
| `RoleBasedPortAssignment`                               | [ ] Implemented| N/A                                      | Group10          |
| `RoleBasedResourceDependency`                           | [ ] Created | N/A                                      | Group26          |
| `RootSwCompositionPrototype`                            | [ ] Implemented| N/A                                      | Group1           |
| `RoughEstimateHeapUsage`                                | [ ] Implemented| N/A                                      | Group23          |
| `RoughEstimateOfExecutionTime`                          | [ ] Implemented| N/A                                      | Group23          |
| `RoughEstimateStackUsage`                               | [ ] Implemented| N/A                                      | Group20          |
| `Row`                                                   | [ ] Implemented| N/A                                      | Group3           |
| `RptAccessEnum`                                         | [ ] Implemented| N/A                                      | Group23          |
| `RptComponent`                                          | [ ] Implemented| N/A                                      | Group23          |
| `RptContainer`                                          | [ ] Created | N/A                                      | Group29          |
| `RptEnablerImplTypeEnum`                                | [ ] Implemented| N/A                                      | Group23          |
| `RptExecutableEntity`                                   | [ ] Implemented| N/A                                      | Group23          |
| `RptExecutableEntityEvent`                              | [ ] Implemented| N/A                                      | Group23          |
| `RptExecutableEntityProperties`                         | [ ] Implemented| N/A                                      | Group23          |
| `RptExecutionContext`                                   | [ ] Implemented| N/A                                      | Group23          |
| `RptExecutionControlEnum`                               | [ ] Implemented| N/A                                      | Group23          |
| `RptHook`                                               | [ ] Created | N/A                                      | Group29          |
| `RptImplPolicy`                                         | [ ] Implemented| N/A                                      | Group23          |
| `RptPreparationEnum`                                    | [ ] Implemented| N/A                                      | Group23          |
| `RptProfile`                                            | [ ] Created | N/A                                      | Group29          |
| `RptServicePoint`                                       | [ ] Implemented| N/A                                      | Group23          |
| `RptServicePointEnum`                                   | [ ] Implemented| N/A                                      | Group23          |
| `RptSupportData`                                        | [ ] Implemented| N/A                                      | Group23          |
| `RptSwPrototypingAccess`                                | [ ] Implemented| N/A                                      | Group23          |
| `RteApiReturnValueProvisionEnum`                        | [ ] Implemented| N/A                                      | Group29          |
| `RteEventInCompositionSeparation`                       | [ ] Created | N/A                                      | Group31          |
| `RteEventInCompositionToOsTaskProxyMapping`             | [ ] Created | N/A                                      | Group31          |
| `RteEventInEcuInstanceRef`                              | [ ] Implemented| N/A                                      | Group12          |
| `RteEventInSystemSeparation`                            | [ ] Created | N/A                                      | Group31          |
| `RteEventInSystemToOsTaskProxyMapping`                  | [ ] Created | N/A                                      | Group31          |
| `RtePluginProps`                                        | [ ] Implemented| N/A                                      | Group6           |
| `RtpTp`                                                 | [ ] Created | N/A                                      | Group32          |
| `RuleArguments`                                         | [ ] Implemented| N/A                                      | Group28          |
| `RuleBasedAxisCont`                                     | [ ] Implemented| N/A                                      | Group28          |
| `RuleBasedValueCont`                                    | [ ] Implemented| N/A                                      | Group28          |
| `RuleBasedValueSpecification`                           | [ ] Implemented| N/A                                      | Group28          |
| `RunMode`                                               | [ ] Implemented| N/A                                      | Group17          |
| `RunnableEntity`                                        | [ ] Implemented| N/A                                      | Group28          |
| `RunnableEntityArgument`                                | [ ] Implemented| N/A                                      | Group2           |
| `RunnableEntityGroup`                                   | [ ] Implemented| N/A                                      | Group28          |
| `RuntimeAddressConfigurationEnum`                       | [ ] Implemented| N/A                                      | Group16          |
| `RuntimeError`                                          | [ ] Implemented| N/A                                      | Group23          |
| `RxAcceptContainedIPduEnum`                             | [ ] Created | N/A                                      | Group31          |
| `RxIdentifierRange`                                     | [ ] Implemented| N/A                                      | Group32          |
| `SOMEIPMessageTypeEnum`                                 | [ ] Implemented| N/A                                      | Group6           |
| `SOMEIPTransformationDescription`                       | [ ] Created | N/A                                      | Group34          |
| `SOMEIPTransformationISignalProps`                      | [ ] Implemented| N/A                                      | Group6           |
| `SOMEIPTransformationProps`                             | [ ] Created | N/A                                      | Group34          |
| `SaveConfigurationEntry`                                | [ ] Implemented| N/A                                      | Group32          |
| `ScaleConstrValidityEnum`                               | [ ] Implemented| N/A                                      | Group9           |
| `ScheduleTableEntry`                                    | [ ] Implemented| N/A                                      | Group31          |
| `SdClientConfig`                                        | [ ] Implemented| N/A                                      | Group7           |
| `SdServerConfig`                                        | [ ] Implemented| N/A                                      | Group16          |
| `SdgAbstractForeignReference`                           | [ ] Implemented| N/A                                      | Group21          |
| `SdgAbstractPrimitiveAttribute`                         | [ ] Implemented| N/A                                      | Group21          |
| `SdgAggregationWithVariation`                           | [ ] Implemented| N/A                                      | Group21          |
| `SdgAttribute`                                          | [ ] Implemented| N/A                                      | Group21          |
| `SdgClass`                                              | [ ] Implemented| N/A                                      | Group21          |
| `SdgDef`                                                | [ ] Implemented| N/A                                      | Group21          |
| `SdgElementWithGid`                                     | [ ] Implemented| N/A                                      | Group21          |
| `SdgForeignReference`                                   | [ ] Implemented| N/A                                      | Group21          |
| `SdgForeignReferenceWithVariation`                      | [ ] Implemented| N/A                                      | Group21          |
| `SdgPrimitiveAttribute`                                 | [ ] Implemented| N/A                                      | Group21          |
| `SdgPrimitiveAttributeWithVariation`                    | [ ] Implemented| N/A                                      | Group21          |
| `SdgReference`                                          | [ ] Implemented| N/A                                      | Group21          |
| `SdgTailoring`                                          | [ ] Created | N/A                                      | Group36          |
| `SecOcCryptoServiceMapping`                             | [ ] Implemented| N/A                                      | Group18          |
| `SectionInitializationPolicyType`                       | [ ] Implemented| N/A                                      | Group22          |
| `SectionNamePrefix`                                     | [ ] Implemented| N/A                                      | Group20          |
| `SecureCommunicationAuthenticationProps`                | [ ] Implemented| N/A                                      | Group31          |
| `SecureCommunicationFreshnessProps`                     | [ ] Implemented| N/A                                      | Group31          |
| `SecureCommunicationProps`                              | [ ] Implemented| N/A                                      | Group31          |
| `SecureCommunicationPropsSet`                           | [ ] Implemented| N/A                                      | Group31          |
| `SecureOnBoardCommunicationNeeds`                       | [ ] Implemented| N/A                                      | Group29          |
| `SecuredIPdu`                                           | [ ] Implemented| N/A                                      | Group15          |
| `SecuredPduHeaderEnum`                                  | [ ] Implemented| N/A                                      | Group15          |
| `SecurityEventAggregationFilter`                        | [ ] Created | N/A                                      | Group36          |
| `SecurityEventContextDataSourceEnum`                    | [ ] Created | N/A                                      | Group36          |
| `SecurityEventContextMapping`                           | [ ] Created | N/A                                      | Group36          |
| `SecurityEventContextMappingApplication`                | [ ] Created | N/A                                      | Group36          |
| `SecurityEventContextMappingBswModule`                  | [ ] Created | N/A                                      | Group36          |
| `SecurityEventContextMappingCommConnector`              | [ ] Created | N/A                                      | Group36          |
| `SecurityEventContextMappingFunctionalCluster`          | [ ] Created | N/A                                      | Group36          |
| `SecurityEventContextProps`                             | [ ] Created | N/A                                      | Group36          |
| `SecurityEventDefinition`                               | [ ] Created | N/A                                      | Group36          |
| `SecurityEventFilterChain`                              | [ ] Created | N/A                                      | Group36          |
| `SecurityEventOneEveryNFilter`                          | [ ] Created | N/A                                      | Group36          |
| `SecurityEventReportingModeEnum`                        | [ ] Created | N/A                                      | Group36          |
| `SecurityEventStateFilter`                              | [ ] Created | N/A                                      | Group36          |
| `SecurityEventThresholdFilter`                          | [ ] Created | N/A                                      | Group36          |
| `SegmentPosition`                                       | [ ] Implemented| N/A                                      | Group15          |
| `SendIndicationEnum`                                    | [ ] Created | N/A                                      | Group34          |
| `SenderAnnotation`                                      | [ ] Created | N/A                                      | Group27          |
| `SenderComSpec`                                         | [ ] Implemented| N/A                                      | Group27          |
| `SenderRecArrayElementMapping`                          | [ ] Implemented| N/A                                      | Group31          |
| `SenderRecArrayTypeMapping`                             | [ ] Implemented| N/A                                      | Group6           |
| `SenderRecCompositeTypeMapping`                         | [ ] Implemented| N/A                                      | Group6           |
| `SenderRecRecordElementMapping`                         | [ ] Implemented| N/A                                      | Group17          |
| `SenderRecRecordTypeMapping`                            | [ ] Implemented| N/A                                      | Group17          |
| `SenderReceiverAnnotation`                              | [ ] Implemented| N/A                                      | Group27          |
| `SenderReceiverCompositeElementToSignalMapping`         | [ ] Created | N/A                                      | Group31          |
| `SenderReceiverInterface`                               | [ ] Implemented| N/A                                      | Group1           |
| `SenderReceiverToSignalGroupMapping`                    | [ ] Implemented| N/A                                      | Group17          |
| `SenderReceiverToSignalMapping`                         | [ ] Implemented| N/A                                      | Group17          |
| `SensorActuatorSwComponentType`                         | [ ] Implemented| N/A                                      | Group29          |
| `SeparateSignalPath`                                    | [ ] Created | N/A                                      | Group31          |
| `ServerArgumentImplPolicyEnum`                          | [ ] Implemented| N/A                                      | Group27          |
| `ServerCallPoint`                                       | [ ] Implemented| N/A                                      | Group2           |
| `ServerComSpec`                                         | [ ] Implemented| N/A                                      | Group27          |
| `ServiceDependency`                                     | [ ] Implemented| N/A                                      | Group23          |
| `ServiceDiagnosticRelevanceEnum`                        | [ ] Implemented| N/A                                      | Group14          |
| `ServiceInstanceCollectionSet`                          | [ ] Created | N/A                                      | Group32          |
| `ServiceNeeds`                                          | [ ] Implemented| N/A                                      | Group4           |
| `ServiceProviderEnum`                                   | [ ] Implemented| N/A                                      | Group27          |
| `ServiceProxySwComponentType`                           | [ ] Implemented| N/A                                      | Group11          |
| `ServiceSwComponentType`                                | [ ] Implemented| N/A                                      | Group29          |
| `SeverityEnum`                                          | [ ] Created | N/A                                      | Group36          |
| `ShortNameFragment`                                     | [ ] Implemented| N/A                                      | Group8           |
| `ShowContentEnum`                                       | [ ] Implemented| N/A                                      | Group3           |
| `ShowResourceAliasNameEnum`                             | [ ] Implemented| N/A                                      | Group3           |
| `ShowResourceCategoryEnum`                              | [ ] Implemented| N/A                                      | Group3           |
| `ShowResourceLongNameEnum`                              | [ ] Implemented| N/A                                      | Group3           |
| `ShowResourceNumberEnum`                                | [ ] Implemented| N/A                                      | Group3           |
| `ShowResourcePageEnum`                                  | [ ] Implemented| N/A                                      | Group3           |
| `ShowResourceShortNameEnum`                             | [ ] Implemented| N/A                                      | Group3           |
| `ShowResourceTypeEnum`                                  | [ ] Implemented| N/A                                      | Group3           |
| `ShowSeeEnum`                                           | [ ] Implemented| N/A                                      | Group3           |
| `SignalFanEnum`                                         | [ ] Implemented| N/A                                      | Group27          |
| `SignalServiceTranslationControlEnum`                   | [ ] Implemented| N/A                                      | Group34          |
| `SignalServiceTranslationElementProps`                  | [ ] Implemented| N/A                                      | Group14          |
| `SignalServiceTranslationEventProps`                    | [ ] Implemented| N/A                                      | Group34          |
| `SignalServiceTranslationProps`                         | [ ] Implemented| N/A                                      | Group34          |
| `SignalServiceTranslationPropsSet`                      | [ ] Implemented| N/A                                      | Group34          |
| `SimulatedExecutionTime`                                | [ ] Implemented| N/A                                      | Group23          |
| `SingleLanguageLongName`                                | [ ] Created | N/A                                      | Group3           |
| `SingleLanguageReferrable`                              | [ ] Implemented| N/A                                      | Group3           |
| `SingleLanguageUnitNames`                               | [ ] Implemented| N/A                                      | Group8           |
| `SlOverviewParagraph`                                   | [ ] Implemented| N/A                                      | Group8           |
| `SlParagraph`                                           | [ ] Implemented| N/A                                      | Group3           |
| `SoAdConfig`                                            | [ ] Implemented| N/A                                      | Group32          |
| `SoAdRoutingGroup`                                      | [ ] Implemented| N/A                                      | Group20          |
| `SoConIPduIdentifier`                                   | [ ] Created | N/A                                      | Group32          |
| `SocketAddress`                                         | [ ] Implemented| N/A                                      | Group32          |
| `SocketConnectionBundle`                                | [ ] Implemented| N/A                                      | Group16          |
| `SocketConnectionIpduIdentifier`                        | [ ] Implemented| N/A                                      | Group16          |
| `SocketConnectionIpduIdentifierSet`                     | [ ] Created | N/A                                      | Group32          |
| `SoftwareContext`                                       | [ ] Implemented| N/A                                      | Group20          |
| `SomeipProtocolRule`                                    | [ ] Implemented| N/A                                      | Group20          |
| `SomeipSdClientEventGroupTimingConfig`                  | [ ] Implemented| N/A                                      | Group32          |
| `SomeipSdRule`                                          | [ ] Implemented| N/A                                      | Group20          |
| `SomeipSdServerEventGroupTimingConfig`                  | [ ] Implemented| N/A                                      | Group32          |
| `SomeipSdServerServiceInstanceConfig`                   | [ ] Created | N/A                                      | Group32          |
| `SomeipTpChannel`                                       | [ ] Created | N/A                                      | Group33          |
| `SomeipTpConfig`                                        | [ ] Created | N/A                                      | Group33          |
| `SomeipTpConnection`                                    | [ ] Created | N/A                                      | Group33          |
| `SpecElementReference`                                  | [ ] Created | N/A                                      | Group36          |
| `SpecElementScope`                                      | [ ] Created | N/A                                      | Group36          |
| `SpecificationDocumentScope`                            | [ ] Created | N/A                                      | Group36          |
| `SpecificationScope`                                    | [ ] Created | N/A                                      | Group36          |
| `SporadicEventTriggering`                               | [ ] Implemented| N/A                                      | Group35          |
| `StackUsage`                                            | [ ] Implemented| N/A                                      | Group20          |
| `StandardNameEnum`                                      | [ ] Implemented| N/A                                      | Group1           |
| `StateDependentFirewall`                                | [ ] Implemented| N/A                                      | Group33          |
| `StaticPart`                                            | [ ] Implemented| N/A                                      | Group5           |
| `StaticSocketConnection`                                | [ ] Implemented| N/A                                      | Group32          |
| `Std`                                                   | [ ] Implemented| N/A                                      | Group3           |
| `StorageConditionStatusEnum`                            | [ ] Implemented| N/A                                      | Group29          |
| `StreamFilterIEEE1722Tp`                                | [ ] Created | N/A                                      | Group30          |
| `StreamFilterIpv4Address`                               | [ ] Created | N/A                                      | Group30          |
| `StreamFilterIpv6Address`                               | [ ] Created | N/A                                      | Group30          |
| `StreamFilterMACAddress`                                | [ ] Created | N/A                                      | Group30          |
| `StreamFilterPortRange`                                 | [ ] Created | N/A                                      | Group30          |
| `StreamFilterRuleDataLinkLayer`                         | [ ] Created | N/A                                      | Group30          |
| `StreamFilterRuleIpTp`                                  | [ ] Created | N/A                                      | Group30          |
| `StructuredReq`                                         | [ ] Implemented| N/A                                      | Group1           |
| `SubElementMapping`                                     | [ ] Implemented| N/A                                      | Group1           |
| `SubElementRef`                                         | [ ] Implemented| N/A                                      | Group1           |
| `Superscript`                                           | [ ] Implemented| N/A                                      | Group21          |
| `SupervisedEntityCheckpointNeeds`                       | [ ] Implemented| N/A                                      | Group4           |
| `SupervisedEntityNeeds`                                 | [ ] Implemented| N/A                                      | Group23          |
| `SupportBufferLockingEnum`                              | [ ] Implemented| N/A                                      | Group2           |
| `SwAddrMethod`                                          | [ ] Implemented| N/A                                      | Group22          |
| `SwAxisCont`                                            | [ ] Created | N/A                                      | Group28          |
| `SwAxisGeneric`                                         | [ ] Implemented| N/A                                      | Group28          |
| `SwAxisGrouped`                                         | [ ] Implemented| N/A                                      | Group3           |
| `SwAxisIndividual`                                      | [ ] Implemented| N/A                                      | Group3           |
| `SwAxisType`                                            | [ ] Created | N/A                                      | Group28          |
| `SwBaseType`                                            | [ ] Implemented| N/A                                      | Group28          |
| `SwBitRepresentation`                                   | [ ] Implemented| N/A                                      | Group28          |
| `SwCalibrationAccessEnum`                               | [ ] Implemented| N/A                                      | Group28          |
| `SwCalprmAxis`                                          | [ ] Implemented| N/A                                      | Group28          |
| `SwCalprmAxisSet`                                       | [ ] Implemented| N/A                                      | Group3           |
| `SwCalprmAxisTypeProps`                                 | [ ] Implemented| N/A                                      | Group28          |
| `SwCalprmRefProxy`                                      | [ ] Implemented| N/A                                      | Group28          |
| `SwComponentDocumentation`                              | [ ] Implemented| N/A                                      | Group29          |
| `SwComponentPrototype`                                  | [ ] Implemented| N/A                                      | Group1           |
| `SwComponentPrototypeAssignment`                        | [ ] Implemented| N/A                                      | Group5           |
| `SwComponentType`                                       | [ ] Implemented| N/A                                      | Group27          |
| `SwConnector`                                           | [ ] Implemented| N/A                                      | Group27          |
| `SwDataDefProps`                                        | [ ] Implemented| N/A                                      | Group28          |
| `SwDataDependency`                                      | [ ] Implemented| N/A                                      | Group28          |
| `SwDataDependencyArgs`                                  | [ ] Implemented| N/A                                      | Group28          |
| `SwGenericAxisParam`                                    | [ ] Implemented| N/A                                      | Group28          |
| `SwGenericAxisParamType`                                | [ ] Implemented| N/A                                      | Group3           |
| `SwImplPolicyEnum`                                      | [ ] Implemented| N/A                                      | Group9           |
| `SwPointerTargetProps`                                  | [ ] Implemented| N/A                                      | Group22          |
| `SwRecordLayout`                                        | [ ] Implemented| N/A                                      | Group3           |
| `SwRecordLayoutGroup`                                   | [ ] Implemented| N/A                                      | Group3           |
| `SwRecordLayoutGroupContent`                            | [ ] Implemented| N/A                                      | Group3           |
| `SwRecordLayoutV`                                       | [ ] Implemented| N/A                                      | Group3           |
| `SwServiceArg`                                          | [ ] Implemented| N/A                                      | Group22          |
| `SwServiceImplPolicyEnum`                               | [ ] Implemented| N/A                                      | Group22          |
| `SwSystemconst`                                         | [ ] Implemented| N/A                                      | Group9           |
| `SwSystemconstDependentFormula`                         | [ ] Implemented| N/A                                      | Group8           |
| `SwSystemconstValue`                                    | [ ] Implemented| N/A                                      | Group8           |
| `SwSystemconstantValueSet`                              | [ ] Implemented| N/A                                      | Group36          |
| `SwTextProps`                                           | [ ] Implemented| N/A                                      | Group28          |
| `SwValueCont`                                           | [ ] Implemented| N/A                                      | Group3           |
| `SwValues`                                              | [ ] Implemented| N/A                                      | Group28          |
| `SwVariableRefProxy`                                    | [ ] Implemented| N/A                                      | Group28          |
| `SwcBswMapping`                                         | [ ] Implemented| N/A                                      | Group1           |
| `SwcBswRunnableMapping`                                 | [ ] Implemented| N/A                                      | Group13          |
| `SwcBswSynchronizedModeGroupPrototype`                  | [ ] Implemented| N/A                                      | Group13          |
| `SwcBswSynchronizedTrigger`                             | [ ] Implemented| N/A                                      | Group13          |
| `SwcExclusiveAreaPolicy`                                | [ ] Implemented| N/A                                      | Group29          |
| `SwcImplementation`                                     | [ ] Implemented| N/A                                      | Group10          |
| `SwcInternalBehavior`                                   | [ ] Implemented| N/A                                      | Group2           |
| `SwcModeManagerErrorEvent`                              | [ ] Created | N/A                                      | Group29          |
| `SwcModeSwitchEvent`                                    | [ ] Implemented| N/A                                      | Group28          |
| `SwcServiceDependency`                                  | [ ] Implemented| N/A                                      | Group29          |
| `SwcSupportedFeature`                                   | [ ] Implemented| N/A                                      | Group2           |
| `SwcTiming`                                             | [ ] Implemented| N/A                                      | Group35          |
| `SwcToApplicationPartitionMapping`                      | [ ] Created | N/A                                      | Group30          |
| `SwcToEcuMapping`                                       | [ ] Implemented| N/A                                      | Group18          |
| `SwcToImplMapping`                                      | [ ] Implemented| N/A                                      | Group18          |
| `SwcToSwcOperationArguments`                            | [ ] Created | N/A                                      | Group31          |
| `SwcToSwcOperationArgumentsDirectionEnum`               | [ ] Created | N/A                                      | Group31          |
| `SwcToSwcSignal`                                        | [ ] Created | N/A                                      | Group31          |
| `SwitchAsynchronousTrafficShaperGroupEntry`             | [ ] Created | N/A                                      | Group30          |
| `SwitchFlowMeteringEntry`                               | [ ] Created | N/A                                      | Group30          |
| `SwitchStreamFilterActionDestPortModification`          | [ ] Created | N/A                                      | Group30          |
| `SwitchStreamFilterActionPortModificationEnum`          | [ ] Created | N/A                                      | Group30          |
| `SwitchStreamFilterEntry`                               | [ ] Created | N/A                                      | Group30          |
| `SwitchStreamFilterRule`                                | [ ] Created | N/A                                      | Group30          |
| `SwitchStreamGateEntry`                                 | [ ] Created | N/A                                      | Group30          |
| `SwitchStreamIdentification`                            | [ ] Created | N/A                                      | Group30          |
| `SymbolProps`                                           | [ ] Implemented| N/A                                      | Group2           |
| `SymbolString`                                          | [ ] Implemented| N/A                                      | Group21          |
| `SymbolicNameProps`                                     | [ ] Implemented| N/A                                      | Group29          |
| `SyncTimeBaseMgrUserNeeds`                              | [ ] Implemented| N/A                                      | Group4           |
| `SynchronizationPointConstraint`                        | [ ] Implemented| N/A                                      | Group35          |
| `SynchronizationTimingConstraint`                       | [ ] Implemented| N/A                                      | Group8           |
| `SynchronizationTypeEnum`                               | [ ] Implemented| N/A                                      | Group35          |
| `SynchronousServerCallPoint`                            | [ ] Implemented| N/A                                      | Group2           |
| `System`                                                | [ ] Implemented| N/A                                      | Group5           |
| `SystemMapping`                                         | [ ] Implemented| N/A                                      | Group30          |
| `SystemSignal`                                          | [ ] Implemented| N/A                                      | Group15          |
| `SystemSignalGroup`                                     | [ ] Implemented| N/A                                      | Group31          |
| `SystemSignalGroupToCommunicationResourceMapping`       | [ ] Created | N/A                                      | Group31          |
| `SystemSignalToCommunicationResourceMapping`            | [ ] Created | N/A                                      | Group31          |
| `SystemTiming`                                          | [ ] Created | N/A                                      | Group35          |
| `TDCpSoftwareClusterMapping`                            | [ ] Created | N/A                                      | Group35          |
| `TDCpSoftwareClusterMappingSet`                         | [ ] Created | N/A                                      | Group35          |
| `TDCpSoftwareClusterResourceMapping`                    | [ ] Created | N/A                                      | Group35          |
| `TDEventBswInternalBehavior`                            | [ ] Implemented| N/A                                      | Group35          |
| `TDEventBswInternalBehaviorTypeEnum`                    | [ ] Implemented| N/A                                      | Group35          |
| `TDEventBswModeDeclaration`                             | [ ] Implemented| N/A                                      | Group35          |
| `TDEventBswModeDeclarationTypeEnum`                     | [ ] Implemented| N/A                                      | Group35          |
| `TDEventBswModule`                                      | [ ] Implemented| N/A                                      | Group35          |
| `TDEventBswModuleTypeEnum`                              | [ ] Implemented| N/A                                      | Group35          |
| `TDEventCom`                                            | [ ] Implemented| N/A                                      | Group35          |
| `TDEventComplex`                                        | [ ] Implemented| N/A                                      | Group35          |
| `TDEventCycleStart`                                     | [ ] Implemented| N/A                                      | Group35          |
| `TDEventFrClusterCycleStart`                            | [ ] Implemented| N/A                                      | Group35          |
| `TDEventFrame`                                          | [ ] Implemented| N/A                                      | Group35          |
| `TDEventFrameEthernet`                                  | [ ] Implemented| N/A                                      | Group35          |
| `TDEventFrameEthernetTypeEnum`                          | [ ] Implemented| N/A                                      | Group35          |
| `TDEventFrameTypeEnum`                                  | [ ] Implemented| N/A                                      | Group35          |
| `TDEventIPdu`                                           | [ ] Implemented| N/A                                      | Group35          |
| `TDEventIPduTypeEnum`                                   | [ ] Implemented| N/A                                      | Group35          |
| `TDEventISignal`                                        | [ ] Implemented| N/A                                      | Group35          |
| `TDEventISignalTypeEnum`                                | [ ] Implemented| N/A                                      | Group35          |
| `TDEventModeDeclaration`                                | [ ] Implemented| N/A                                      | Group35          |
| `TDEventModeDeclarationTypeEnum`                        | [ ] Implemented| N/A                                      | Group35          |
| `TDEventOccurrenceExpression`                           | [ ] Implemented| N/A                                      | Group35          |
| `TDEventOccurrenceExpressionFormula`                    | [ ] Implemented| N/A                                      | Group35          |
| `TDEventOperation`                                      | [ ] Implemented| N/A                                      | Group35          |
| `TDEventOperationTypeEnum`                              | [ ] Implemented| N/A                                      | Group35          |
| `TDEventSLLETPort`                                      | [ ] Implemented| N/A                                      | Group35          |
| `TDEventSwc`                                            | [ ] Implemented| N/A                                      | Group35          |
| `TDEventSwcInternalBehavior`                            | [ ] Implemented| N/A                                      | Group35          |
| `TDEventSwcInternalBehaviorReference`                   | [ ] Implemented| N/A                                      | Group35          |
| `TDEventSwcInternalBehaviorTypeEnum`                    | [ ] Implemented| N/A                                      | Group35          |
| `TDEventTTCanCycleStart`                                | [ ] Implemented| N/A                                      | Group35          |
| `TDEventTrigger`                                        | [ ] Implemented| N/A                                      | Group35          |
| `TDEventTriggerTypeEnum`                                | [ ] Implemented| N/A                                      | Group35          |
| `TDEventVariableDataPrototype`                          | [ ] Implemented| N/A                                      | Group35          |
| `TDEventVariableDataPrototypeTypeEnum`                  | [ ] Implemented| N/A                                      | Group35          |
| `TDEventVfb`                                            | [ ] Implemented| N/A                                      | Group8           |
| `TDEventVfbPort`                                        | [ ] Implemented| N/A                                      | Group35          |
| `TDEventVfbReference`                                   | [ ] Implemented| N/A                                      | Group35          |
| `TDHeaderIdRange`                                       | [ ] Implemented| N/A                                      | Group35          |
| `Table`                                                 | [ ] Implemented| N/A                                      | Group3           |
| `TableSeparatorString`                                  | [ ] Implemented| N/A                                      | Group3           |
| `TargetIPduRef`                                         | [ ] Implemented| N/A                                      | Group17          |
| `Tbody`                                                 | [ ] Implemented| N/A                                      | Group3           |
| `TcpIpIcmpv4Props`                                      | [ ] Implemented| N/A                                      | Group5           |
| `TcpIpIcmpv6Props`                                      | [ ] Implemented| N/A                                      | Group5           |
| `TcpOptionFilterList`                                   | [ ] Implemented| N/A                                      | Group16          |
| `TcpOptionFilterSet`                                    | [ ] Implemented| N/A                                      | Group16          |
| `TcpProps`                                              | [ ] Implemented| N/A                                      | Group5           |
| `TcpRule`                                               | [ ] Implemented| N/A                                      | Group20          |
| `TcpTp`                                                 | [ ] Implemented| N/A                                      | Group16          |
| `TcpUdpConfig`                                          | [ ] Implemented| N/A                                      | Group6           |
| `TextTableMapping`                                      | [ ] Implemented| N/A                                      | Group1           |
| `TextTableValuePair`                                    | [ ] Implemented| N/A                                      | Group27          |
| `TextValueSpecification`                                | [ ] Implemented| N/A                                      | Group9           |
| `TextualCondition`                                      | [ ] Created | N/A                                      | Group36          |
| `Tgroup`                                                | [ ] Implemented| N/A                                      | Group3           |
| `TimeRangeType`                                         | [ ] Implemented| N/A                                      | Group15          |
| `TimeRangeTypeTolerance`                                | [ ] Implemented| N/A                                      | Group15          |
| `TimeSyncClientConfiguration`                           | [ ] Implemented| N/A                                      | Group6           |
| `TimeSyncServerConfiguration`                           | [ ] Implemented| N/A                                      | Group16          |
| `TimeSyncTechnologyEnum`                                | [ ] Implemented| N/A                                      | Group32          |
| `TimeSynchronization`                                   | [ ] Implemented| N/A                                      | Group16          |
| `TimeValue`                                             | [ ] Implemented| N/A                                      | Group27          |
| `TimeValueValueVariationPoint`                          | [ ] Implemented| N/A                                      | Group8           |
| `TimingCondition`                                       | [ ] Implemented| N/A                                      | Group35          |
| `TimingConditionFormula`                                | [ ] Implemented| N/A                                      | Group35          |
| `TimingDescriptionEventChain`                           | [ ] Implemented| N/A                                      | Group8           |
| `TimingEvent`                                           | [ ] Implemented| N/A                                      | Group28          |
| `TimingExtensionResource`                               | [ ] Implemented| N/A                                      | Group35          |
| `TimingModeInstance`                                    | [ ] Implemented| N/A                                      | Group35          |
| `TlsCryptoCipherSuite`                                  | [ ] Implemented| N/A                                      | Group6           |
| `TlsCryptoCipherSuiteProps`                             | [ ] Implemented| N/A                                      | Group6           |
| `TlsCryptoServiceMapping`                               | [ ] Implemented| N/A                                      | Group6           |
| `TlsPskIdentity`                                        | [ ] Implemented| N/A                                      | Group6           |
| `TlsVersionEnum`                                        | [ ] Implemented| N/A                                      | Group6           |
| `TlvDataIdDefinition`                                   | [ ] Implemented| N/A                                      | Group6           |
| `TlvDataIdDefinitionSet`                                | [ ] Implemented| N/A                                      | Group6           |
| `Topic1`                                                | [ ] Implemented| N/A                                      | Group22          |
| `TopicContent`                                          | [ ] Implemented| N/A                                      | Group3           |
| `TopicContentOrMsrQuery`                                | [ ] Implemented| N/A                                      | Group9           |
| `TopicOrMsrQuery`                                       | [ ] Implemented| N/A                                      | Group22          |
| `TpAddress`                                             | [ ] Implemented| N/A                                      | Group18          |
| `TpConfig`                                              | [ ] Implemented| N/A                                      | Group33          |
| `TpConnection`                                          | [ ] Implemented| N/A                                      | Group33          |
| `TpConnectionIdent`                                     | [ ] Implemented| N/A                                      | Group23          |
| `TpPort`                                                | [ ] Implemented| N/A                                      | Group16          |
| `Traceable`                                             | [ ] Implemented| N/A                                      | Group21          |
| `TraceableTable`                                        | [ ] Implemented| N/A                                      | Group3           |
| `TraceableText`                                         | [ ] Implemented| N/A                                      | Group1           |
| `TracedFailure`                                         | [ ] Implemented| N/A                                      | Group23          |
| `TransferPropertyEnum`                                  | [ ] Implemented| N/A                                      | Group15          |
| `TransformationComSpecProps`                            | [ ] Implemented| N/A                                      | Group27          |
| `TransformationDescription`                             | [ ] Implemented| N/A                                      | Group28          |
| `TransformationISignalProps`                            | [ ] Implemented| N/A                                      | Group6           |
| `TransformationProps`                                   | [ ] Created | N/A                                      | Group34          |
| `TransformationPropsSet`                                | [ ] Created | N/A                                      | Group34          |
| `TransformationTechnology`                              | [ ] Implemented| N/A                                      | Group27          |
| `TransformerClassEnum`                                  | [ ] Implemented| N/A                                      | Group28          |
| `TransformerHardErrorEvent`                             | [ ] Created | N/A                                      | Group28          |
| `TransmissionAcknowledgementRequest`                    | [ ] Implemented| N/A                                      | Group27          |
| `TransmissionComSpecProps`                              | [ ] Implemented| N/A                                      | Group27          |
| `TransmissionModeCondition`                             | [ ] Implemented| N/A                                      | Group15          |
| `TransmissionModeDeclaration`                           | [ ] Implemented| N/A                                      | Group15          |
| `TransmissionModeDefinitionEnum`                        | [ ] Implemented| N/A                                      | Group27          |
| `TransmissionModeTiming`                                | [ ] Implemented| N/A                                      | Group15          |
| `TransportLayerRule`                                    | [ ] Implemented| N/A                                      | Group20          |
| `TransportProtocolConfiguration`                        | [ ] Implemented| N/A                                      | Group6           |
| `Trigger`                                               | [ ] Implemented| N/A                                      | Group1           |
| `TriggerIPduSendCondition`                              | [ ] Implemented| N/A                                      | Group15          |
| `TriggerInAtomicSwcInstanceRef`                         | [ ] Implemented| N/A                                      | Group11          |
| `TriggerInterface`                                      | [ ] Implemented| N/A                                      | Group1           |
| `TriggerInterfaceMapping`                               | [ ] Implemented| N/A                                      | Group1           |
| `TriggerMapping`                                        | [ ] Implemented| N/A                                      | Group1           |
| `TriggerMode`                                           | [ ] Implemented| N/A                                      | Group15          |
| `TriggerPortAnnotation`                                 | [ ] Implemented| N/A                                      | Group27          |
| `TriggerToSignalMapping`                                | [ ] Created | N/A                                      | Group31          |
| `Tt`                                                    | [ ] Implemented| N/A                                      | Group21          |
| `TtcanAbsolutelyScheduledTiming`                        | [ ] Implemented| N/A                                      | Group32          |
| `TtcanCluster`                                          | [ ] Created | N/A                                      | Group29          |
| `TtcanCommunicationConnector`                           | [ ] Created | N/A                                      | Group29          |
| `TtcanCommunicationController`                          | [ ] Created | N/A                                      | Group29          |
| `TtcanPhysicalChannel`                                  | [ ] Created | N/A                                      | Group29          |
| `TtcanTriggerType`                                      | [ ] Implemented| N/A                                      | Group32          |
| `UdpChecksumCalculationEnum`                            | [ ] Implemented| N/A                                      | Group32          |
| `UdpNmCluster`                                          | [ ] Implemented| N/A                                      | Group18          |
| `UdpNmClusterCoupling`                                  | [ ] Implemented| N/A                                      | Group18          |
| `UdpNmEcu`                                              | [ ] Implemented| N/A                                      | Group6           |
| `UdpNmNode`                                             | [ ] Implemented| N/A                                      | Group18          |
| `UdpProps`                                              | [ ] Implemented| N/A                                      | Group5           |
| `UdpRule`                                               | [ ] Implemented| N/A                                      | Group20          |
| `UdpTp`                                                 | [ ] Implemented| N/A                                      | Group16          |
| `UnassignFrameId`                                       | [ ] Implemented| N/A                                      | Group31          |
| `Unit`                                                  | [ ] Implemented| N/A                                      | Group28          |
| `UnitGroup`                                             | [ ] Implemented| N/A                                      | Group9           |
| `UnlimitedIntegerValueVariationPoint`                   | [ ] Implemented| N/A                                      | Group8           |
| `UriString`                                             | [ ] Implemented| N/A                                      | Group21          |
| `Url`                                                   | [ ] Implemented| N/A                                      | Group3           |
| `UserDefinedCluster`                                    | [ ] Created | N/A                                      | Group30          |
| `UserDefinedCommunicationConnector`                     | [ ] Created | N/A                                      | Group30          |
| `UserDefinedCommunicationController`                    | [ ] Created | N/A                                      | Group30          |
| `UserDefinedEthernetFrame`                              | [ ] Created | N/A                                      | Group33          |
| `UserDefinedGlobalTimeMaster`                           | [ ] Created | N/A                                      | Group34          |
| `UserDefinedGlobalTimeSlave`                            | [ ] Created | N/A                                      | Group34          |
| `UserDefinedIPdu`                                       | [ ] Implemented| N/A                                      | Group15          |
| `UserDefinedPdu`                                        | [ ] Implemented| N/A                                      | Group15          |
| `UserDefinedPhysicalChannel`                            | [ ] Created | N/A                                      | Group30          |
| `UserDefinedTransformationComSpecProps`                 | [ ] Implemented| N/A                                      | Group5           |
| `UserDefinedTransformationDescription`                  | [ ] Created | N/A                                      | Group34          |
| `UserDefinedTransformationISignalProps`                 | [ ] Implemented| N/A                                      | Group6           |
| `UserDefinedTransformationProps`                        | [ ] Created | N/A                                      | Group34          |
| `V2xDataManagerNeeds`                                   | [ ] Implemented| N/A                                      | Group5           |
| `V2xFacUserNeeds`                                       | [ ] Implemented| N/A                                      | Group5           |
| `V2xMUserNeeds`                                         | [ ] Implemented| N/A                                      | Group5           |
| `ValignEnum`                                            | [ ] Implemented| N/A                                      | Group3           |
| `ValueGroup`                                            | [ ] Implemented| N/A                                      | Group28          |
| `ValueList`                                             | [ ] Implemented| N/A                                      | Group28          |
| `ValueRestrictionWithSeverity`                          | [ ] Created | N/A                                      | Group36          |
| `ValueSpecification`                                    | [ ] Implemented| N/A                                      | Group28          |
| `VariableAccess`                                        | [ ] Implemented| N/A                                      | Group12          |
| `VariableAccessInEcuInstanceRef`                        | [ ] Implemented| N/A                                      | Group12          |
| `VariableAccessScopeEnum`                               | [ ] Implemented| N/A                                      | Group12          |
| `VariableAndParameterInterfaceMapping`                  | [ ] Implemented| N/A                                      | Group11          |
| `VariableDataPrototype`                                 | [ ] Implemented| N/A                                      | Group2           |
| `VariableDataPrototypeInSystemInstanceRef`              | [ ] Implemented| N/A                                      | Group7           |
| `VariableInAtomicSWCTypeInstanceRef`                    | [ ] Implemented| N/A                                      | Group2           |
| `VariableInAtomicSwcInstanceRef`                        | [ ] Implemented| N/A                                      | Group2           |
| `VariationPoint`                                        | [ ] Implemented| N/A                                      | Group8           |
| `VariationPointProxy`                                   | [ ] Implemented| N/A                                      | Group29          |
| `VariationRestrictionWithSeverity`                      | [ ] Created | N/A                                      | Group36          |
| `VendorSpecificServiceNeeds`                            | [ ] Implemented| N/A                                      | Group5           |
| `VerbatimStringPlain`                                   | [ ] Implemented| N/A                                      | Group21          |
| `VerificationStatusIndicationModeEnum`                  | [ ] Implemented| N/A                                      | Group29          |
| `VfbTiming`                                             | [ ] Created | N/A                                      | Group35          |
| `ViewMap`                                               | [ ] Implemented| N/A                                      | Group22          |
| `ViewMapSet`                                            | [ ] Implemented| N/A                                      | Group22          |
| `ViewTokens`                                            | [ ] Implemented| N/A                                      | Group3           |
| `VlanConfig`                                            | [ ] Implemented| N/A                                      | Group16          |
| `VlanMembership`                                        | [ ] Implemented| N/A                                      | Group6           |
| `WaitPoint`                                             | [ ] Implemented| N/A                                      | Group28          |
| `WarningIndicatorRequestedBitNeeds`                     | [ ] Implemented| N/A                                      | Group5           |
| `WhitespaceControlled`                                  | [ ] Implemented| N/A                                      | Group8           |
| `WorstCaseHeapUsage`                                    | [ ] Implemented| N/A                                      | Group22          |
| `WorstCaseStackUsage`                                   | [ ] Implemented| N/A                                      | Group20          |
| `Xdoc`                                                  | [ ] Implemented| N/A                                      | Group3           |
| `Xfile`                                                 | [ ] Implemented| N/A                                      | Group3           |
| `XmlSpaceEnum`                                          | [ ] Implemented| N/A                                      | Group8           |
| `Xref`                                                  | [ ] Implemented| N/A                                      | Group3           |
| `XrefTarget`                                            | [ ] Implemented| N/A                                      | Group3           |
