# All Sync Todo Classes by Group

Generated from all Group files in `docs/plan/sync-todo/` — Classes ordered by group, then by appearance order within each group.

Status `*` (or an explicit `Deferred` status) = sync complete (Steps 1–8) but the `# Spec verified:`/`# XSD verified:` stamp is **deferred to a batch 9b user confirmation** (audited 2026-09-27 against the src stamps).
Class-availability lifecycle: `Created` = the class exists in src as an empty stub (dependency placeholder — e.g. the Group21–36 stub pass) but implementation has not started · `Implemented` = the class exists in src with members but the queued 9-step sync is not complete · `Pending` = the class is not available (not defined in src at all).


## Group1

Status: **2/75** completed

| Class Name                              | Status          | Commit ID  |
| --------------------------------------- | --------------- | ---------- |
| `ARObject`                              | [x] Done        | 1be5c3b284 |
| `ARElement`                             | [x] Done        | 9c8dd7a    |
| `ReferenceBase`                         | [ ] Implemented | N/A        |
| `MultilanguageReferrable`               | [ ] Implemented | N/A        |
| `HwPin`                                 | [ ] Implemented | N/A        |
| `HwPinGroup`                            | [ ] Implemented | N/A        |
| `HwType`                                | [ ] Implemented | N/A        |
| `HwElement`                             | [ ] Implemented | N/A        |
| `FirewallRule`                          | [ ] Implemented | N/A        |
| `PortInterfaceBlueprintMapping`         | [ ] Implemented | N/A        |
| `PortPrototypeBlueprintMapping`         | [ ] Implemented | N/A        |
| `BlueprintMappingSet`                   | [ ] Implemented | N/A        |
| `ConstantSpecificationMappingSet`       | [ ] Implemented | N/A        |
| `StandardNameEnum`                      | [ ] Implemented | N/A        |
| `StructuredReq`                         | [ ] Implemented | N/A        |
| `TraceableText`                         | [ ] Implemented | N/A        |
| `Identifiable`                          | [ ] Implemented | N/A        |
| `CollectableElement`                    | [ ] Implemented | N/A        |
| `PackageableElement`                    | [ ] Implemented | N/A        |
| `ARPackage`                             | [ ] Implemented | N/A        |
| `AUTOSAR`                               | [ ] Implemented | N/A        |
| `FileInfoComment`                       | [ ] Implemented | N/A        |
| `AutoCollectEnum`                       | [ ] Implemented | N/A        |
| `Collection`                            | [ ] Implemented | N/A        |
| `AtpType`                               | [ ] Implemented | N/A        |
| `AtpPrototype`                          | [ ] Implemented | N/A        |
| `PortPrototype`                         | [ ] Implemented | N/A        |
| `DataPrototype`                         | [ ] Implemented | N/A        |
| `ModeDeclarationGroupPrototype`         | [ ] Implemented | N/A        |
| `RootSwCompositionPrototype`            | [ ] Implemented | N/A        |
| `SwComponentPrototype`                  | [ ] Implemented | N/A        |
| `AtpStructureElement`                   | [ ] Implemented | N/A        |
| `AtpDefinition`                         | [ ] Implemented | N/A        |
| `BlueprintPolicy`                       | [ ] Implemented | N/A        |
| `AtpBlueprint`                          | [ ] Implemented | N/A        |
| `AtpBlueprintable`                      | [ ] Implemented | N/A        |
| `AtpBlueprintMapping`                   | [ ] Implemented | N/A        |
| `ApplicationDeferredDataType`           | [ ] Implemented | N/A        |
| `PortInterfaceMapping`                  | [ ] Implemented | N/A        |
| `AutosarDataType`                       | [ ] Implemented | N/A        |
| `AbstractImplementationDataType`        | [ ] Implemented | N/A        |
| `AbstractImplementationDataTypeElement` | [ ] Implemented | N/A        |
| `DataInterface`                         | [ ] Implemented | N/A        |
| `ParameterInterface`                    | [ ] Implemented | N/A        |
| `NvDataInterface`                       | [ ] Implemented | N/A        |
| `HandleInvalidEnum`                     | [ ] Implemented | N/A        |
| `InvalidationPolicy`                    | [ ] Implemented | N/A        |
| `SenderReceiverInterface`               | [ ] Implemented | N/A        |
| `Trigger`                               | [ ] Implemented | N/A        |
| `TriggerInterface`                      | [ ] Implemented | N/A        |
| `TriggerMapping`                        | [ ] Implemented | N/A        |
| `TriggerInterfaceMapping`               | [ ] Implemented | N/A        |
| `IdentCaption`                          | [ ] Implemented | N/A        |
| `ModeAccessPointIdent`                  | [ ] Implemented | N/A        |
| `ModeDeclarationMappingSet`             | [ ] Implemented | N/A        |
| `SubElementRef`                         | [ ] Implemented | N/A        |
| `TextTableMapping`                      | [ ] Implemented | N/A        |
| `SubElementMapping`                     | [ ] Implemented | N/A        |
| `FlatInstanceDescriptor`                | [ ] Implemented | N/A        |
| `FlatMap`                               | [ ] Implemented | N/A        |
| `ProgramminglanguageEnum`               | [ ] Implemented | N/A        |
| `Compiler`                              | [ ] Implemented | N/A        |
| `Linker`                                | [ ] Implemented | N/A        |
| `Code`                                  | [ ] Implemented | N/A        |
| `DependencyOnArtifact`                  | [ ] Implemented | N/A        |
| `ResourceConsumption`                   | [ ] Implemented | N/A        |
| `SwcBswMapping`                         | [ ] Implemented | N/A        |
| `BuildActionInvocator`                  | [ ] Implemented | N/A        |
| `BuildActionEntity`                     | [ ] Implemented | N/A        |
| `BuildEngineeringObject`                | [ ] Implemented | N/A        |
| `BuildActionIoElement`                  | [ ] Implemented | N/A        |
| `BuildActionEnvironment`                | [ ] Implemented | N/A        |
| `BuildAction`                           | [ ] Implemented | N/A        |
| `BuildActionManifest`                   | [ ] Implemented | N/A        |
| `Implementation`                        | [ ] Implemented | N/A        |

## Group2

Status: **0/44** completed

| Class Name                                              | Status          | Commit ID |
| ------------------------------------------------------- | --------------- | --------- |
| `PortInterfaceMappingSet`                               | [ ] Implemented | N/A       |
| `MetaDataItem`                                          | [ ] Implemented | N/A       |
| `MetaDataItemSet`                                       | [ ] Implemented | N/A       |
| `ApplicationCompositeElementInPortInterfaceInstanceRef` | [ ] Implemented | N/A       |
| `SymbolProps`                                           | [ ] Implemented | N/A       |
| `PPortPrototype`                                        | [ ] Implemented | N/A       |
| `RPortPrototype`                                        | [ ] Implemented | N/A       |
| `PRPortPrototype`                                       | [ ] Implemented | N/A       |
| `PortGroup`                                             | [ ] Implemented | N/A       |
| `InnerPortGroupInCompositionInstanceRef`                | [ ] Implemented | N/A       |
| `RTEEvent`                                              | [ ] Implemented | N/A       |
| `ServerCallPoint`                                       | [ ] Implemented | N/A       |
| `VariableDataPrototype`                                 | [ ] Implemented | N/A       |
| `PerInstanceMemory`                                     | [ ] Implemented | N/A       |
| `PortInCompositionTypeInstanceRef`                      | [ ] Implemented | N/A       |
| `AssemblySwConnector`                                   | [ ] Implemented | N/A       |
| `DataTypeMappingSet`                                    | [ ] Implemented | N/A       |
| `ApplicationDataType`                                   | [ ] Implemented | N/A       |
| `ApplicationCompositeElementDataPrototype`              | [ ] Implemented | N/A       |
| `InitEvent`                                             | [ ] Implemented | N/A       |
| `BackgroundEvent`                                       | [ ] Implemented | N/A       |
| `SynchronousServerCallPoint`                            | [ ] Implemented | N/A       |
| `AsynchronousServerCallPoint`                           | [ ] Implemented | N/A       |
| `AsynchronousServerCallResultPoint`                     | [ ] Implemented | N/A       |
| `VariableInAtomicSwcInstanceRef`                        | [ ] Implemented | N/A       |
| `VariableInAtomicSWCTypeInstanceRef`                    | [ ] Implemented | N/A       |
| `ArVariableInImplementationDataInstanceRef`             | [ ] Implemented | N/A       |
| `DelegationSwConnector`                                 | [ ] Implemented | N/A       |
| `ApplicationPrimitiveDataType`                          | [ ] Implemented | N/A       |
| `ApplicationCompositeDataType`                          | [ ] Implemented | N/A       |
| `ApplicationRecordElement`                              | [ ] Implemented | N/A       |
| `RunnableEntityArgument`                                | [ ] Implemented | N/A       |
| `ExternalTriggeringPointIdent`                          | [ ] Implemented | N/A       |
| `PortDefinedArgumentValue`                              | [ ] Implemented | N/A       |
| `CompositionSwComponentType`                            | [ ] Implemented | N/A       |
| `ApplicationRecordDataType`                             | [ ] Implemented | N/A       |
| `DataTransformationErrorHandlingEnum`                   | [ ] Implemented | N/A       |
| `DataTransformationStatusForwardingEnum`                | [ ] Implemented | N/A       |
| `SwcSupportedFeature`                                   | [ ] Implemented | N/A       |
| `CommunicationBufferLocking`                            | [ ] Implemented | N/A       |
| `SupportBufferLockingEnum`                              | [ ] Implemented | N/A       |
| `PortAPIOption`                                         | [ ] Implemented | N/A       |
| `IncludedModeDeclarationGroupSet`                       | [ ] Implemented | N/A       |
| `SwcInternalBehavior`                                   | [ ] Implemented | N/A       |

## Group3

Status: **0/95** completed

| Class Name                             | Status          | Commit ID |
| -------------------------------------- | --------------- | --------- |
| `ChapterEnumBreak`                     | [ ] Implemented | N/A       |
| `KeepWithPreviousEnum`                 | [ ] Implemented | N/A       |
| `Paginateable`                         | [ ] Implemented | N/A       |
| `MultilanguageLongName`                | [ ] Implemented | N/A       |
| `GraphicFitEnum`                       | [ ] Implemented | N/A       |
| `GraphicNotationEnum`                  | [ ] Implemented | N/A       |
| `Graphic`                              | [ ] Implemented | N/A       |
| `SingleLanguageLongName`               | [ ] Created     | N/A       |
| `SingleLanguageReferrable`             | [ ] Implemented | N/A       |
| `MimeTypeString`                       | [ ] Implemented | N/A       |
| `Url`                                  | [ ] Implemented | N/A       |
| `Br`                                   | [ ] Implemented | N/A       |
| `Std`                                  | [ ] Implemented | N/A       |
| `Xdoc`                                 | [ ] Implemented | N/A       |
| `Xfile`                                | [ ] Implemented | N/A       |
| `XrefTarget`                           | [ ] Implemented | N/A       |
| `ResolutionPolicyEnum`                 | [ ] Implemented | N/A       |
| `ShowContentEnum`                      | [ ] Implemented | N/A       |
| `ShowResourceAliasNameEnum`            | [ ] Implemented | N/A       |
| `ShowResourceCategoryEnum`             | [ ] Implemented | N/A       |
| `ShowResourceLongNameEnum`             | [ ] Implemented | N/A       |
| `ShowResourceNumberEnum`               | [ ] Implemented | N/A       |
| `ShowResourcePageEnum`                 | [ ] Implemented | N/A       |
| `ShowResourceShortNameEnum`            | [ ] Implemented | N/A       |
| `ShowResourceTypeEnum`                 | [ ] Implemented | N/A       |
| `ShowSeeEnum`                          | [ ] Implemented | N/A       |
| `Xref`                                 | [ ] Implemented | N/A       |
| `MixedContentForParagraph`             | [ ] Implemented | N/A       |
| `SlParagraph`                          | [ ] Implemented | N/A       |
| `LParagraph`                           | [ ] Implemented | N/A       |
| `FrameEnum`                            | [ ] Implemented | N/A       |
| `AlignEnum`                            | [ ] Implemented | N/A       |
| `ValignEnum`                           | [ ] Implemented | N/A       |
| `OrientEnum`                           | [ ] Implemented | N/A       |
| `TableSeparatorString`                 | [ ] Implemented | N/A       |
| `NameTokens`                           | [ ] Implemented | N/A       |
| `ViewTokens`                           | [ ] Implemented | N/A       |
| `DocumentViewSelectable`               | [ ] Implemented | N/A       |
| `Colspec`                              | [ ] Implemented | N/A       |
| `Entry`                                | [ ] Implemented | N/A       |
| `Row`                                  | [ ] Implemented | N/A       |
| `Tbody`                                | [ ] Implemented | N/A       |
| `Tgroup`                               | [ ] Implemented | N/A       |
| `Table`                                | [ ] Implemented | N/A       |
| `TraceableTable`                       | [ ] Implemented | N/A       |
| `TopicContent`                         | [ ] Implemented | N/A       |
| `MultiLanguageParagraph`               | [ ] Implemented | N/A       |
| `AreaEnumNohref`                       | [ ] Implemented | N/A       |
| `AreaEnumShape`                        | [ ] Implemented | N/A       |
| `Area`                                 | [ ] Implemented | N/A       |
| `Map`                                  | [ ] Implemented | N/A       |
| `LGraphic`                             | [ ] Implemented | N/A       |
| `MlFigure`                             | [ ] Implemented | N/A       |
| `MsrQueryResultChapter`                | [ ] Implemented | N/A       |
| `MsrQueryChapter`                      | [ ] Implemented | N/A       |
| `MsrQueryResultTopic1`                 | [ ] Implemented | N/A       |
| `MsrQueryTopic1`                       | [ ] Implemented | N/A       |
| `MsrQueryP1`                           | [ ] Implemented | N/A       |
| `CompuContent`                         | [ ] Implemented | N/A       |
| `CompuConstContent`                    | [ ] Implemented | N/A       |
| `CompuConstTextContent`                | [ ] Implemented | N/A       |
| `CompuConstNumericContent`             | [ ] Implemented | N/A       |
| `CompuConstFormulaContent`             | [ ] Implemented | N/A       |
| `CompuConst`                           | [ ] Implemented | N/A       |
| `CompuScaleContents`                   | [ ] Implemented | N/A       |
| `CompuNominatorDenominator`            | [ ] Implemented | N/A       |
| `CompuRationalCoeffs`                  | [ ] Implemented | N/A       |
| `CompuScaleRationalFormula`            | [ ] Implemented | N/A       |
| `CompuScaleConstantContents`           | [ ] Implemented | N/A       |
| `Compu`                                | [ ] Implemented | N/A       |
| `CompuMethod`                          | [ ] Implemented | N/A       |
| `CompuScale`                           | [ ] Implemented | N/A       |
| `CompuScales`                          | [ ] Implemented | N/A       |
| `DataConstrRule`                       | [ ] Implemented | N/A       |
| `DataConstr`                           | [ ] Implemented | N/A       |
| `CompositeValueSpecification`          | [ ] Implemented | N/A       |
| `ArrayValueSpecification`              | [ ] Implemented | N/A       |
| `RecordValueSpecification`             | [ ] Implemented | N/A       |
| `CompositeRuleBasedValueArgument`      | [ ] Implemented | N/A       |
| `CompositeRuleBasedValueSpecification` | [ ] Implemented | N/A       |
| `SwValueCont`                          | [ ] Implemented | N/A       |
| `SwCalprmAxisSet`                      | [ ] Implemented | N/A       |
| `SwAxisIndividual`                     | [ ] Implemented | N/A       |
| `SwAxisGrouped`                        | [ ] Implemented | N/A       |
| `SwRecordLayoutGroupContent`           | [ ] Implemented | N/A       |
| `SwGenericAxisParamType`               | [ ] Implemented | N/A       |
| `SwRecordLayoutV`                      | [ ] Implemented | N/A       |
| `SwRecordLayout`                       | [ ] Implemented | N/A       |
| `AsamRecordLayoutSemantics`            | [ ] Created     | N/A       |
| `RecordLayoutIteratorPoint`            | [ ] Created     | N/A       |
| `SwRecordLayoutGroup`                  | [ ] Implemented | N/A       |
| `GeneralAnnotation`                    | [ ] Implemented | N/A       |
| `FirewallActionEnum`                   | [ ] Implemented | N/A       |
| `CompuGenericMath`                     | [ ] Implemented | N/A       |
| `Ref`                                  | [ ] Implemented | N/A       |

## Group4

Status: **0/29** completed

| Class Name                              | Status          | Commit ID |
| --------------------------------------- | --------------- | --------- |
| `BswPerInstanceMemoryPolicy`            | [ ] Implemented | N/A       |
| `BswClientPolicy`                       | [ ] Implemented | N/A       |
| `BswInternalTriggeringPointPolicy`      | [ ] Implemented | N/A       |
| `BswParameterPolicy`                    | [ ] Implemented | N/A       |
| `BswReleasedTriggerPolicy`              | [ ] Implemented | N/A       |
| `BswDataSendPolicy`                     | [ ] Implemented | N/A       |
| `BswInternalBehavior`                   | [ ] Implemented | N/A       |
| `ServiceNeeds`                          | [ ] Implemented | N/A       |
| `DiagEventDebounceAlgorithm`            | [ ] Implemented | N/A       |
| `DiagEventDebounceMonitorInternal`      | [ ] Implemented | N/A       |
| `EcuStateMgrUserNeeds`                  | [ ] Implemented | N/A       |
| `DltUserNeeds`                          | [ ] Implemented | N/A       |
| `DiagnosticComponentNeeds`              | [ ] Implemented | N/A       |
| `DiagnosticUploadDownloadNeeds`         | [ ] Implemented | N/A       |
| `DiagnosticsCommunicationSecurityNeeds` | [ ] Implemented | N/A       |
| `FunctionInhibitionNeeds`               | [ ] Implemented | N/A       |
| `GlobalSupervisionNeeds`                | [ ] Implemented | N/A       |
| `HardwareTestNeeds`                     | [ ] Implemented | N/A       |
| `SupervisedEntityCheckpointNeeds`       | [ ] Implemented | N/A       |
| `SyncTimeBaseMgrUserNeeds`              | [ ] Implemented | N/A       |
| `BswMgrNeeds`                           | [ ] Implemented | N/A       |
| `CryptoKeyManagementNeeds`              | [ ] Implemented | N/A       |
| `CryptoServiceJobNeeds`                 | [ ] Implemented | N/A       |
| `DiagnosticControlNeeds`                | [ ] Implemented | N/A       |
| `DiagnosticEventManagerNeeds`           | [ ] Implemented | N/A       |
| `DiagnosticRequestFileTransferNeeds`    | [ ] Implemented | N/A       |
| `DoIpActivationLineNeeds`               | [ ] Implemented | N/A       |
| `DoIpGidNeeds`                          | [ ] Implemented | N/A       |
| `DoIpGidSynchronizationNeeds`           | [ ] Implemented | N/A       |

## Group5

Status: **0/70** completed

| Class Name                              | Status          | Commit ID |
| --------------------------------------- | --------------- | --------- |
| `DoIpPowerModeStatusNeeds`              | [ ] Implemented | N/A       |
| `FurtherActionByteNeeds`                | [ ] Implemented | N/A       |
| `IdsMgrCustomTimestampNeeds`            | [ ] Implemented | N/A       |
| `J1939DcmDm19Support`                   | [ ] Implemented | N/A       |
| `J1939RmIncomingRequestServiceNeeds`    | [ ] Implemented | N/A       |
| `J1939RmOutgoingRequestServiceNeeds`    | [ ] Implemented | N/A       |
| `V2xDataManagerNeeds`                   | [ ] Implemented | N/A       |
| `V2xFacUserNeeds`                       | [ ] Implemented | N/A       |
| `V2xMUserNeeds`                         | [ ] Implemented | N/A       |
| `VendorSpecificServiceNeeds`            | [ ] Implemented | N/A       |
| `WarningIndicatorRequestedBitNeeds`     | [ ] Implemented | N/A       |
| `OperationInSystemInstanceRef`          | [ ] Implemented | N/A       |
| `ClientIdDefinition`                    | [ ] Implemented | N/A       |
| `ClientIdDefinitionSet`                 | [ ] Implemented | N/A       |
| `InterpolationRoutine`                  | [ ] Implemented | N/A       |
| `InterpolationRoutineMapping`           | [ ] Implemented | N/A       |
| `InterpolationRoutineMappingSet`        | [ ] Implemented | N/A       |
| `SwComponentPrototypeAssignment`        | [ ] Implemented | N/A       |
| `CpSoftwareCluster`                     | [ ] Implemented | N/A       |
| `System`                                | [ ] Implemented | N/A       |
| `J1939Cluster`                          | [ ] Implemented | N/A       |
| `J1939SharedAddressCluster`             | [ ] Implemented | N/A       |
| `PortGroupInSystemInstanceRef`          | [ ] Implemented | N/A       |
| `ComManagementMapping`                  | [ ] Implemented | N/A       |
| `TcpProps`                              | [ ] Implemented | N/A       |
| `UdpProps`                              | [ ] Implemented | N/A       |
| `EthTcpIpProps`                         | [ ] Implemented | N/A       |
| `TcpIpIcmpv4Props`                      | [ ] Implemented | N/A       |
| `TcpIpIcmpv6Props`                      | [ ] Implemented | N/A       |
| `EthTcpIpIcmpProps`                     | [ ] Implemented | N/A       |
| `EcuPartition`                          | [ ] Implemented | N/A       |
| `OsTaskPreemptabilityEnum`              | [ ] Implemented | N/A       |
| `OsTaskProxy`                           | [ ] Implemented | N/A       |
| `PdurIPduGroup`                         | [ ] Implemented | N/A       |
| `DoIpRoutingActivation`                 | [ ] Implemented | N/A       |
| `DoIpInterface`                         | [ ] Implemented | N/A       |
| `DoIpConfig`                            | [ ] Implemented | N/A       |
| `ConsumedProvidedServiceInstanceGroup`  | [ ] Implemented | N/A       |
| `ClientIdRange`                         | [ ] Implemented | N/A       |
| `PrivacyLevel`                          | [ ] Implemented | N/A       |
| `DltArgument`                           | [ ] Implemented | N/A       |
| `DltMessage`                            | [ ] Implemented | N/A       |
| `DltContext`                            | [ ] Implemented | N/A       |
| `DltApplication`                        | [ ] Implemented | N/A       |
| `DltEcu`                                | [ ] Implemented | N/A       |
| `DltDefaultTraceStateEnum`              | [ ] Implemented | N/A       |
| `LogTraceDefaultLogLevelEnum`           | [ ] Implemented | N/A       |
| `DltLogChannel`                         | [ ] Implemented | N/A       |
| `DltConfig`                             | [ ] Implemented | N/A       |
| `EcuInstance`                           | [ ] Implemented | N/A       |
| `DiagnosticConnection`                  | [ ] Implemented | N/A       |
| `EthernetPhysicalChannel`               | [ ] Implemented | N/A       |
| `FrameTriggering`                       | [ ] Implemented | N/A       |
| `ContainedIPduCollectionSemanticsEnum`  | [ ] Implemented | N/A       |
| `PduCollectionTriggerEnum`              | [ ] Implemented | N/A       |
| `ContainedIPduProps`                    | [ ] Implemented | N/A       |
| `ModeDrivenTransmissionModeCondition`   | [ ] Implemented | N/A       |
| `StaticPart`                            | [ ] Implemented | N/A       |
| `DynamicPartAlternative`                | [ ] Implemented | N/A       |
| `GeneralPurposePdu`                     | [ ] Implemented | N/A       |
| `GeneralPurposeIPdu`                    | [ ] Implemented | N/A       |
| `CycleCounter`                          | [ ] Implemented | N/A       |
| `CycleRepetitionType`                   | [ ] Implemented | N/A       |
| `CycleRepetition`                       | [ ] Implemented | N/A       |
| `CommunicationCycle`                    | [ ] Implemented | N/A       |
| `FramePort`                             | [ ] Implemented | N/A       |
| `QueuedSenderComSpec`                   | [ ] Implemented | N/A       |
| `UserDefinedTransformationComSpecProps` | [ ] Implemented | N/A       |
| `EndToEndProtectionVariablePrototype`   | [ ] Implemented | N/A       |
| `EndToEndProtectionSet`                 | [ ] Implemented | N/A       |

## Group6

Status: **0/45** completed

| Class Name                              | Status          | Commit ID |
| --------------------------------------- | --------------- | --------- |
| `AbstractEthernetFrame`                 | [ ] Implemented | N/A       |
| `GenericEthernetFrame`                  | [ ] Implemented | N/A       |
| `CouplingPortStructuralElement`         | [ ] Implemented | N/A       |
| `CouplingPortScheduler`                 | [ ] Implemented | N/A       |
| `VlanMembership`                        | [ ] Implemented | N/A       |
| `NetworkEndpointAddress`                | [ ] Implemented | N/A       |
| `OrderedMaster`                         | [ ] Implemented | N/A       |
| `TimeSyncClientConfiguration`           | [ ] Implemented | N/A       |
| `TransportProtocolConfiguration`        | [ ] Implemented | N/A       |
| `TcpUdpConfig`                          | [ ] Implemented | N/A       |
| `FlexrayFrame`                          | [ ] Implemented | N/A       |
| `CryptoServiceMapping`                  | [ ] Implemented | N/A       |
| `TlsVersionEnum`                        | [ ] Implemented | N/A       |
| `TlsPskIdentity`                        | [ ] Implemented | N/A       |
| `CryptoServicePrimitive`                | [ ] Implemented | N/A       |
| `TlsCryptoCipherSuiteProps`             | [ ] Implemented | N/A       |
| `CryptoEllipticCurveProps`              | [ ] Implemented | N/A       |
| `CryptoSignatureScheme`                 | [ ] Implemented | N/A       |
| `CryptoCertificateAlgorithmFamilyEnum`  | [ ] Implemented | N/A       |
| `CryptoCertificateFormatEnum`           | [ ] Implemented | N/A       |
| `CryptoServiceCertificate`              | [ ] Implemented | N/A       |
| `TlsCryptoCipherSuite`                  | [ ] Implemented | N/A       |
| `TlsCryptoServiceMapping`               | [ ] Implemented | N/A       |
| `DataTransformationSet`                 | [ ] Implemented | N/A       |
| `DataPrototypeTransformationProps`      | [ ] Implemented | N/A       |
| `TransformationISignalProps`            | [ ] Implemented | N/A       |
| `SOMEIPMessageTypeEnum`                 | [ ] Implemented | N/A       |
| `TlvDataIdDefinition`                   | [ ] Implemented | N/A       |
| `TlvDataIdDefinitionSet`                | [ ] Implemented | N/A       |
| `SOMEIPTransformationISignalProps`      | [ ] Implemented | N/A       |
| `UserDefinedTransformationISignalProps` | [ ] Implemented | N/A       |
| `SenderRecCompositeTypeMapping`         | [ ] Implemented | N/A       |
| `SenderRecArrayTypeMapping`             | [ ] Implemented | N/A       |
| `NmClusterCoupling`                     | [ ] Implemented | N/A       |
| `NmCluster`                             | [ ] Implemented | N/A       |
| `FlexrayNmCluster`                      | [ ] Implemented | N/A       |
| `FlexrayNmEcu`                          | [ ] Implemented | N/A       |
| `FlexrayNmNode`                         | [ ] Implemented | N/A       |
| `UdpNmEcu`                              | [ ] Implemented | N/A       |
| `J1939NmCluster`                        | [ ] Implemented | N/A       |
| `J1939NmEcu`                            | [ ] Implemented | N/A       |
| `NmConfig`                              | [ ] Implemented | N/A       |
| `IPduMapping`                           | [ ] Implemented | N/A       |
| `PduMappingDefaultValue`                | [ ] Implemented | N/A       |
| `RtePluginProps`                        | [ ] Implemented | N/A       |

## Group7

Status: **0/32** completed

| Class Name                                    | Status          | Commit ID |
| --------------------------------------------- | --------------- | --------- |
| `ComponentInCompositionInstanceRef`           | [ ] Implemented | N/A       |
| `SdClientConfig`                              | [ ] Implemented | N/A       |
| `HwAttributeDef`                              | [ ] Implemented | N/A       |
| `HwCategory`                                  | [ ] Implemented | N/A       |
| `HwAttributeValue`                            | [ ] Implemented | N/A       |
| `HwAttributeLiteralDef`                       | [ ] Implemented | N/A       |
| `CryptoKeySlot`                               | [ ] Implemented | N/A       |
| `AbstractDoIpLogicAddressProps`               | [ ] Implemented | N/A       |
| `DoIpLogicTargetAddressProps`                 | [ ] Implemented | N/A       |
| `DoIpLogicTesterAddressProps`                 | [ ] Implemented | N/A       |
| `DoIpTpConfig`                                | [ ] Implemented | N/A       |
| `FirewallRuleProps`                           | [ ] Implemented | N/A       |
| `IdsPlatformInstantiation`                    | [ ] Implemented | N/A       |
| `IdsmModuleInstantiation`                     | [ ] Implemented | N/A       |
| `PlatformModuleEthernetEndpointConfiguration` | [ ] Implemented | N/A       |
| `CommunicationControllerMapping`              | [ ] Implemented | N/A       |
| `HwPortMapping`                               | [ ] Implemented | N/A       |
| `ECUMapping`                                  | [ ] Implemented | N/A       |
| `VariableDataPrototypeInSystemInstanceRef`    | [ ] Implemented | N/A       |
| `ComponentInSystemInstanceRef`                | [ ] Implemented | N/A       |
| `PortPrototypeBlueprintInitValue`             | [ ] Implemented | N/A       |
| `PortPrototypeBlueprint`                      | [ ] Implemented | N/A       |
| `Keyword`                                     | [ ] Implemented | N/A       |
| `KeywordSet`                                  | [ ] Implemented | N/A       |
| `DiagnosticServiceInstance`                   | [ ] Implemented | N/A       |
| `DiagnosticServiceTable`                      | [ ] Implemented | N/A       |
| `DiagnosticCommonElement`                     | [ ] Implemented | N/A       |
| `DiagnosticAuthRoleProxy`                     | [ ] Implemented | N/A       |
| `DiagnosticSession`                           | [ ] Implemented | N/A       |
| `DiagnosticSecurityLevel`                     | [ ] Implemented | N/A       |
| `DiagnosticEnvironmentalCondition`            | [ ] Implemented | N/A       |
| `DiagnosticAccessPermission`                  | [ ] Implemented | N/A       |

## Group8

Status: **0/47** completed

| Class Name                               | Status          | Commit ID |
| ---------------------------------------- | --------------- | --------- |
| `BindingTimeEnum`                        | [ ] Implemented | N/A       |
| `XmlSpaceEnum`                           | [ ] Implemented | N/A       |
| `ShortNameFragment`                      | [ ] Implemented | N/A       |
| `MultidimensionalTime`                   | [ ] Implemented | N/A       |
| `LifeCyclePeriod`                        | [ ] Implemented | N/A       |
| `AttributeValueVariationPoint`           | [ ] Implemented | N/A       |
| `FormulaExpression`                      | [ ] Implemented | N/A       |
| `SwSystemconstDependentFormula`          | [ ] Implemented | N/A       |
| `ConditionByFormula`                     | [ ] Implemented | N/A       |
| `MixedContentForOverviewParagraph`       | [ ] Implemented | N/A       |
| `WhitespaceControlled`                   | [ ] Implemented | N/A       |
| `MixedContentForPlainText`               | [ ] Implemented | N/A       |
| `MixedContentForVerbatim`                | [ ] Implemented | N/A       |
| `SlOverviewParagraph`                    | [ ] Implemented | N/A       |
| `MixedContentForUnitNames`               | [ ] Implemented | N/A       |
| `SingleLanguageUnitNames`                | [ ] Implemented | N/A       |
| `SwSystemconstValue`                     | [ ] Implemented | N/A       |
| `PostBuildVariantCondition`              | [ ] Implemented | N/A       |
| `PostBuildVariantCriterion`              | [ ] Implemented | N/A       |
| `PostBuildVariantCriterionValue`         | [ ] Implemented | N/A       |
| `OffsetTimingConstraint`                 | [ ] Implemented | N/A       |
| `SynchronizationTimingConstraint`        | [ ] Implemented | N/A       |
| `TimingDescriptionEventChain`            | [ ] Implemented | N/A       |
| `AutosarOperationArgumentInstance`       | [ ] Implemented | N/A       |
| `TDEventVfb`                             | [ ] Implemented | N/A       |
| `BlueprintGenerator`                     | [ ] Implemented | N/A       |
| `BlueprintMapping`                       | [ ] Implemented | N/A       |
| `LifeCycleInfo`                          | [ ] Implemented | N/A       |
| `LifeCycleInfoSet`                       | [ ] Implemented | N/A       |
| `VariationPoint`                         | [ ] Implemented | N/A       |
| `ModeInSwcBswInstanceRef`                | [ ] Implemented | N/A       |
| `ModeInSwcInstanceRef`                   | [ ] Implemented | N/A       |
| `AbstractEnumerationValueVariationPoint` | [ ] Implemented | N/A       |
| `AbstractNumericalVariationPoint`        | [ ] Implemented | N/A       |
| `BooleanValueVariationPoint`             | [ ] Implemented | N/A       |
| `FloatValueVariationPoint`               | [ ] Implemented | N/A       |
| `IntegerValueVariationPoint`             | [ ] Implemented | N/A       |
| `LimitValueVariationPoint`               | [ ] Implemented | N/A       |
| `NumericalValueVariationPoint`           | [ ] Implemented | N/A       |
| `PositiveIntegerValueVariationPoint`     | [ ] Implemented | N/A       |
| `TimeValueValueVariationPoint`           | [ ] Implemented | N/A       |
| `UnlimitedIntegerValueVariationPoint`    | [ ] Implemented | N/A       |
| `BlueprintFormula`                       | [ ] Implemented | N/A       |
| `FMConditionByFeaturesAndAttributes`     | [ ] Implemented | N/A       |
| `FMConditionByFeaturesAndSwSystemconsts` | [ ] Implemented | N/A       |
| `FMFormulaByFeaturesAndAttributes`       | [ ] Implemented | N/A       |
| `FMFormulaByFeaturesAndSwSystemconsts`   | [ ] Implemented | N/A       |

## Group9

Status: **0/29** completed

| Class Name                    | Status          | Commit ID |
| ----------------------------- | --------------- | --------- |
| `NumericalValueSpecification` | [ ] Implemented | N/A       |
| `TextValueSpecification`      | [ ] Implemented | N/A       |
| `ConstantReference`           | [ ] Implemented | N/A       |
| `ConstantSpecification`       | [ ] Implemented | N/A       |
| `DataFilterTypeEnum`          | [ ] Implemented | N/A       |
| `DataFilter`                  | [ ] Implemented | N/A       |
| `Modification`                | [ ] Implemented | N/A       |
| `ScaleConstrValidityEnum`     | [ ] Implemented | N/A       |
| `UnitGroup`                   | [ ] Implemented | N/A       |
| `SwImplPolicyEnum`            | [ ] Implemented | N/A       |
| `SwSystemconst`               | [ ] Implemented | N/A       |
| `ListEnum`                    | [ ] Implemented | N/A       |
| `Item`                        | [ ] Implemented | N/A       |
| `TopicContentOrMsrQuery`      | [ ] Implemented | N/A       |
| `LOverviewParagraph`          | [ ] Implemented | N/A       |
| `LPlainText`                  | [ ] Implemented | N/A       |
| `LVerbatim`                   | [ ] Implemented | N/A       |
| `ARList`                      | [ ] Implemented | N/A       |
| `ChapterContent`              | [ ] Implemented | N/A       |
| `ChapterModel`                | [ ] Implemented | N/A       |
| `PrmCharContents`             | [ ] Implemented | N/A       |
| `PrmCharNumericalValue`       | [ ] Implemented | N/A       |
| `PrmCharAbsTol`               | [ ] Implemented | N/A       |
| `PrmCharMinTypMax`            | [ ] Implemented | N/A       |
| `PrmCharNumericalContents`    | [ ] Implemented | N/A       |
| `PrmCharTextualContents`      | [ ] Implemented | N/A       |
| `PrmChar`                     | [ ] Implemented | N/A       |
| `GeneralParameter`            | [ ] Implemented | N/A       |
| `Prms`                        | [ ] Implemented | N/A       |

## Group10

Status: **0/32** completed

| Class Name                         | Status          | Commit ID |
| ---------------------------------- | --------------- | --------- |
| `DependencyUsageEnum`              | [ ] Implemented | N/A       |
| `ArrayImplPolicyEnum`              | [ ] Implemented | N/A       |
| `ApiPrincipleEnum`                 | [ ] Implemented | N/A       |
| `ReentrancyLevelEnum`              | [ ] Implemented | N/A       |
| `ImplementationProps`              | [ ] Implemented | N/A       |
| `PerInstanceMemorySize`            | [ ] Implemented | N/A       |
| `SwcImplementation`                | [ ] Implemented | N/A       |
| `ImplementationDataTypeElement`    | [ ] Implemented | N/A       |
| `ReceptionComSpecProps`            | [ ] Implemented | N/A       |
| `CompositeNetworkRepresentation`   | [ ] Implemented | N/A       |
| `ModeSwitchedAckRequest`           | [ ] Implemented | N/A       |
| `ModeSwitchReceiverComSpec`        | [ ] Implemented | N/A       |
| `ModeSwitchSenderComSpec`          | [ ] Implemented | N/A       |
| `NvProvideComSpec`                 | [ ] Implemented | N/A       |
| `NvRequireComSpec`                 | [ ] Implemented | N/A       |
| `ParameterRequireComSpec`          | [ ] Implemented | N/A       |
| `QueuedReceiverComSpec`            | [ ] Implemented | N/A       |
| `DataTypeMap`                      | [ ] Implemented | N/A       |
| `EndToEndDescription`              | [ ] Implemented | N/A       |
| `ModeSwitchEventTriggeredActivity` | [ ] Implemented | N/A       |
| `AutosarVariableRef`               | [ ] Implemented | N/A       |
| `RoleBasedPortAssignment`          | [ ] Implemented | N/A       |
| `AutosarParameterRef`              | [ ] Implemented | N/A       |
| `NvBlockNeedsReliabilityEnum`      | [ ] Implemented | N/A       |
| `NvBlockNeedsWritingPriorityEnum`  | [ ] Implemented | N/A       |
| `RamBlockStatusControlEnum`        | [ ] Implemented | N/A       |
| `NvBlockDataMapping`               | [ ] Implemented | N/A       |
| `BulkNvDataDescriptor`             | [ ] Implemented | N/A       |
| `RoleBasedDataAssignment`          | [ ] Implemented | N/A       |
| `InstantiationDataDefProps`        | [ ] Implemented | N/A       |
| `NvBlockNeeds`                     | [ ] Implemented | N/A       |
| `NvBlockDescriptor`                | [ ] Implemented | N/A       |

## Group11

Status: **0/24** completed

| Class Name                             | Status          | Commit ID |
| -------------------------------------- | --------------- | --------- |
| `ModeActivationKind`                   | [ ] Implemented | N/A       |
| `ModeDeclarationGroupPrototypeMapping` | [ ] Implemented | N/A       |
| `ModeRequestTypeMap`                   | [ ] Implemented | N/A       |
| `ClientServerApplicationErrorMapping`  | [ ] Implemented | N/A       |
| `ClientServerOperationMapping`         | [ ] Implemented | N/A       |
| `ClientServerInterfaceMapping`         | [ ] Implemented | N/A       |
| `ModeInterfaceMapping`                 | [ ] Implemented | N/A       |
| `VariableAndParameterInterfaceMapping` | [ ] Implemented | N/A       |
| `Field`                                | [ ] Implemented | N/A       |
| `AbstractProvidedPortPrototype`        | [ ] Implemented | N/A       |
| `AbstractRequiredPortPrototype`        | [ ] Implemented | N/A       |
| `ServiceProxySwComponentType`          | [ ] Implemented | N/A       |
| `ModeGroupInAtomicSwcInstanceRef`      | [ ] Implemented | N/A       |
| `OperationInAtomicSwcInstanceRef`      | [ ] Implemented | N/A       |
| `RModeInAtomicSwcInstanceRef`          | [ ] Implemented | N/A       |
| `TriggerInAtomicSwcInstanceRef`        | [ ] Implemented | N/A       |
| `PModeGroupInAtomicSwcInstanceRef`     | [ ] Implemented | N/A       |
| `RModeGroupInAtomicSWCInstanceRef`     | [ ] Implemented | N/A       |
| `POperationInAtomicSwcInstanceRef`     | [ ] Implemented | N/A       |
| `ROperationInAtomicSwcInstanceRef`     | [ ] Implemented | N/A       |
| `RVariableInAtomicSwcInstanceRef`      | [ ] Implemented | N/A       |
| `PTriggerInAtomicSwcTypeInstanceRef`   | [ ] Implemented | N/A       |
| `PPortInCompositionInstanceRef`        | [ ] Implemented | N/A       |
| `RPortInCompositionInstanceRef`        | [ ] Implemented | N/A       |

## Group12

Status: **0/15** completed

| Class Name                           | Status          | Commit ID |
| ------------------------------------ | --------------- | --------- |
| `ParameterAccess`                    | [ ] Implemented | N/A       |
| `VariableAccessScopeEnum`            | [ ] Implemented | N/A       |
| `VariableAccess`                     | [ ] Implemented | N/A       |
| `InternalTriggeringPoint`            | [ ] Implemented | N/A       |
| `ModeAccessPoint`                    | [ ] Implemented | N/A       |
| `ModeSwitchPoint`                    | [ ] Implemented | N/A       |
| `AsynchronousServerCallReturnsEvent` | [ ] Implemented | N/A       |
| `DataReceiveErrorEvent`              | [ ] Implemented | N/A       |
| `DataReceivedEvent`                  | [ ] Implemented | N/A       |
| `DataSendCompletedEvent`             | [ ] Implemented | N/A       |
| `DataWriteCompletedEvent`            | [ ] Implemented | N/A       |
| `InternalTriggerOccurredEvent`       | [ ] Implemented | N/A       |
| `OperationInvokedEvent`              | [ ] Implemented | N/A       |
| `RteEventInEcuInstanceRef`           | [ ] Implemented | N/A       |
| `VariableAccessInEcuInstanceRef`     | [ ] Implemented | N/A       |

## Group13

Status: **0/23** completed

| Class Name                              | Status          | Commit ID |
| --------------------------------------- | --------------- | --------- |
| `BswApiOptions`                         | [ ] Implemented | N/A       |
| `BswModuleCallPoint`                    | [ ] Implemented | N/A       |
| `BswDirectCallPoint`                    | [ ] Implemented | N/A       |
| `BswSynchronousServerCallPoint`         | [ ] Implemented | N/A       |
| `BswInternalTriggeringPoint`            | [ ] Implemented | N/A       |
| `BswInterruptEntity`                    | [ ] Implemented | N/A       |
| `BswModeSwitchAckRequest`               | [ ] Implemented | N/A       |
| `BswDataReceptionPolicy`                | [ ] Implemented | N/A       |
| `BswQueuedDataReceptionPolicy`          | [ ] Implemented | N/A       |
| `BswAsynchronousServerCallReturnsEvent` | [ ] Implemented | N/A       |
| `BswDataReceivedEvent`                  | [ ] Implemented | N/A       |
| `BswInternalTriggerOccurredEvent`       | [ ] Implemented | N/A       |
| `BswModeManagerErrorEvent`              | [ ] Implemented | N/A       |
| `BswModeSwitchedAckEvent`               | [ ] Implemented | N/A       |
| `BswTimingEvent`                        | [ ] Implemented | N/A       |
| `BswEntryRelationshipEnum`              | [ ] Implemented | N/A       |
| `BswEntryRelationship`                  | [ ] Implemented | N/A       |
| `BswEntryRelationshipSet`               | [ ] Implemented | N/A       |
| `BswModuleClientServerEntry`            | [ ] Implemented | N/A       |
| `BswModuleDependency`                   | [ ] Implemented | N/A       |
| `SwcBswRunnableMapping`                 | [ ] Implemented | N/A       |
| `SwcBswSynchronizedModeGroupPrototype`  | [ ] Implemented | N/A       |
| `SwcBswSynchronizedTrigger`             | [ ] Implemented | N/A       |

## Group14

Status: **0/25** completed

| Class Name                                 | Status          | Commit ID |
| ------------------------------------------ | --------------- | --------- |
| `DiagnosticAudienceEnum`                   | [ ] Implemented | N/A       |
| `DiagnosticClearDtcNotificationEnum`       | [ ] Implemented | N/A       |
| `DiagnosticProcessingStyleEnum`            | [ ] Implemented | N/A       |
| `DiagnosticRoutineTypeEnum`                | [ ] Implemented | N/A       |
| `DiagnosticServiceRequestCallbackTypeEnum` | [ ] Implemented | N/A       |
| `DiagnosticValueAccessEnum`                | [ ] Implemented | N/A       |
| `DtcFormatTypeEnum`                        | [ ] Implemented | N/A       |
| `DtcKindEnum`                              | [ ] Implemented | N/A       |
| `ServiceDiagnosticRelevanceEnum`           | [ ] Implemented | N/A       |
| `DiagnosticCapabilityElement`              | [ ] Implemented | N/A       |
| `DiagnosticCommunicationManagerNeeds`      | [ ] Implemented | N/A       |
| `DiagnosticEventInfoNeeds`                 | [ ] Implemented | N/A       |
| `DiagnosticRoutineNeeds`                   | [ ] Implemented | N/A       |
| `DiagnosticValueNeeds`                     | [ ] Implemented | N/A       |
| `DtcStatusChangeNotificationNeeds`         | [ ] Implemented | N/A       |
| `CryptoServiceNeeds`                       | [ ] Implemented | N/A       |
| `DiagEventDebounceCounterBased`            | [ ] Implemented | N/A       |
| `SignalServiceTranslationElementProps`     | [ ] Implemented | N/A       |
| `DiagnosticServiceClass`                   | [ ] Implemented | N/A       |
| `DiagnosticJumpToBootLoaderEnum`           | [ ] Implemented | N/A       |
| `DiagnosticLogicalOperatorEnum`            | [ ] Implemented | N/A       |
| `DiagnosticEnvConditionFormulaPart`        | [ ] Implemented | N/A       |
| `DiagnosticEnvConditionFormula`            | [ ] Implemented | N/A       |
| `DiagnosticEnvCompareCondition`            | [ ] Implemented | N/A       |
| `DiagnosticEnvModeElement`                 | [ ] Implemented | N/A       |

## Group15

Status: **0/24** completed

| Class Name                    | Status          | Commit ID |
| ----------------------------- | --------------- | --------- |
| `CommunicationDirectionType`  | [ ] Implemented | N/A       |
| `TransferPropertyEnum`        | [ ] Implemented | N/A       |
| `MultiplexedPart`             | [ ] Implemented | N/A       |
| `DynamicPart`                 | [ ] Implemented | N/A       |
| `SegmentPosition`             | [ ] Implemented | N/A       |
| `ISignalPort`                 | [ ] Implemented | N/A       |
| `ISignalIPduGroup`            | [ ] Implemented | N/A       |
| `MultiplexedIPdu`             | [ ] Implemented | N/A       |
| `TriggerMode`                 | [ ] Implemented | N/A       |
| `SecuredIPdu`                 | [ ] Implemented | N/A       |
| `SecuredPduHeaderEnum`        | [ ] Implemented | N/A       |
| `UserDefinedIPdu`             | [ ] Implemented | N/A       |
| `UserDefinedPdu`              | [ ] Implemented | N/A       |
| `SystemSignal`                | [ ] Implemented | N/A       |
| `TimeRangeType`               | [ ] Implemented | N/A       |
| `TimeRangeTypeTolerance`      | [ ] Implemented | N/A       |
| `TransmissionModeCondition`   | [ ] Implemented | N/A       |
| `TriggerIPduSendCondition`    | [ ] Implemented | N/A       |
| `CyclicTiming`                | [ ] Implemented | N/A       |
| `EventControlledTiming`       | [ ] Implemented | N/A       |
| `FlexrayChannelName`          | [ ] Implemented | N/A       |
| `PncGatewayTypeEnum`          | [ ] Implemented | N/A       |
| `TransmissionModeTiming`      | [ ] Implemented | N/A       |
| `TransmissionModeDeclaration` | [ ] Implemented | N/A       |

## Group16

Status: **0/29** completed

| Class Name                              | Status          | Commit ID |
| --------------------------------------- | --------------- | --------- |
| `RuntimeAddressConfigurationEnum`       | [ ] Implemented | N/A       |
| `IpAddressKeepEnum`                     | [ ] Implemented | N/A       |
| `Ipv6AddressSourceEnum`                 | [ ] Implemented | N/A       |
| `Ipv4AddressSourceEnum`                 | [ ] Implemented | N/A       |
| `DoIpEntity`                            | [ ] Implemented | N/A       |
| `TpPort`                                | [ ] Implemented | N/A       |
| `InitialSdDelayConfig`                  | [ ] Implemented | N/A       |
| `EthernetPriorityRegeneration`          | [ ] Implemented | N/A       |
| `TimeSyncServerConfiguration`           | [ ] Implemented | N/A       |
| `CouplingPortAbstractShaper`            | [ ] Implemented | N/A       |
| `CouplingPortAsynchronousTrafficShaper` | [ ] Pending     | N/A       |
| `CouplingPortCreditBasedShaper`         | [ ] Pending     | N/A       |
| `MacMulticastGroup`                     | [ ] Implemented | N/A       |
| `IPSecConfig`                           | [ ] Implemented | N/A       |
| `NetworkEndpoint`                       | [ ] Implemented | N/A       |
| `VlanConfig`                            | [ ] Implemented | N/A       |
| `Ipv4Configuration`                     | [ ] Implemented | N/A       |
| `GenericTp`                             | [ ] Implemented | N/A       |
| `TcpTp`                                 | [ ] Implemented | N/A       |
| `UdpTp`                                 | [ ] Implemented | N/A       |
| `PduCollectionSemanticsEnum`            | [ ] Implemented | N/A       |
| `SocketConnectionIpduIdentifier`        | [ ] Implemented | N/A       |
| `SocketConnectionBundle`                | [ ] Implemented | N/A       |
| `RequestResponseDelay`                  | [ ] Implemented | N/A       |
| `SdServerConfig`                        | [ ] Implemented | N/A       |
| `TcpOptionFilterList`                   | [ ] Implemented | N/A       |
| `TcpOptionFilterSet`                    | [ ] Implemented | N/A       |
| `IPv6ExtHeaderFilterList`               | [ ] Implemented | N/A       |
| `TimeSynchronization`                   | [ ] Implemented | N/A       |

## Group17

Status: **0/26** completed

| Class Name                                 | Status          | Commit ID |
| ------------------------------------------ | --------------- | --------- |
| `CanClusterBusOffRecovery`                 | [ ] Implemented | N/A       |
| `CanCommunicationConnector`                | [ ] Implemented | N/A       |
| `CanControllerConfiguration`               | [ ] Implemented | N/A       |
| `CanControllerConfigurationRequirements`   | [ ] Implemented | N/A       |
| `CanControllerFdConfigurationRequirements` | [ ] Implemented | N/A       |
| `ResumePosition`                           | [ ] Implemented | N/A       |
| `ApplicationEntry`                         | [ ] Implemented | N/A       |
| `LinScheduleTable`                         | [ ] Implemented | N/A       |
| `RunMode`                                  | [ ] Implemented | N/A       |
| `LinCommunicationConnector`                | [ ] Implemented | N/A       |
| `FlexrayFrameTriggering`                   | [ ] Implemented | N/A       |
| `FlexrayAbsolutelyScheduledTiming`         | [ ] Implemented | N/A       |
| `FlexrayCommunicationConnector`            | [ ] Implemented | N/A       |
| `FlexrayCommunicationController`           | [ ] Implemented | N/A       |
| `FlexrayPhysicalChannel`                   | [ ] Implemented | N/A       |
| `DataMapping`                              | [ ] Implemented | N/A       |
| `IndexedArrayElement`                      | [ ] Implemented | N/A       |
| `SenderRecRecordElementMapping`            | [ ] Implemented | N/A       |
| `SenderRecRecordTypeMapping`               | [ ] Implemented | N/A       |
| `SenderReceiverToSignalMapping`            | [ ] Implemented | N/A       |
| `SenderReceiverToSignalGroupMapping`       | [ ] Implemented | N/A       |
| `DefaultValueElement`                      | [ ] Implemented | N/A       |
| `FrameMapping`                             | [ ] Implemented | N/A       |
| `ISignalMapping`                           | [ ] Implemented | N/A       |
| `TargetIPduRef`                            | [ ] Implemented | N/A       |
| `Gateway`                                  | [ ] Implemented | N/A       |

## Group18

Status: **0/17** completed

| Class Name                                  | Status          | Commit ID |
| ------------------------------------------- | --------------- | --------- |
| `NmEcu`                                     | [ ] Implemented | N/A       |
| `CanNmCluster`                              | [ ] Implemented | N/A       |
| `UdpNmCluster`                              | [ ] Implemented | N/A       |
| `CanNmNode`                                 | [ ] Implemented | N/A       |
| `UdpNmNode`                                 | [ ] Implemented | N/A       |
| `CanNmClusterCoupling`                      | [ ] Implemented | N/A       |
| `UdpNmClusterCoupling`                      | [ ] Implemented | N/A       |
| `FlexrayNmClusterCoupling`                  | [ ] Implemented | N/A       |
| `SecOcCryptoServiceMapping`                 | [ ] Implemented | N/A       |
| `EndToEndTransformationISignalProps`        | [ ] Implemented | N/A       |
| `TpAddress`                                 | [ ] Implemented | N/A       |
| `LinTpConnection`                           | [ ] Implemented | N/A       |
| `EndToEndProtectionISignalIPdu`             | [ ] Implemented | N/A       |
| `SwcToEcuMapping`                           | [ ] Implemented | N/A       |
| `ApplicationPartitionToEcuPartitionMapping` | [ ] Implemented | N/A       |
| `SwcToImplMapping`                          | [ ] Implemented | N/A       |
| `AppOsTaskProxyToEcuTaskProxyMapping`       | [ ] Implemented | N/A       |

## Group19

Status: **0/16** completed

| Class Name                       | Status          | Commit ID |
| -------------------------------- | --------------- | --------- |
| `ConfigReferenceValue`           | [ ] Implemented | N/A       |
| `EcucValueCollection`            | [ ] Implemented | N/A       |
| `ModuleConfiguration`            | [ ] Implemented | N/A       |
| `EcucConfigurationClassEnum`     | [ ] Implemented | N/A       |
| `EcucScopeEnum`                  | [ ] Implemented | N/A       |
| `EcucDestinationUriDefRefType`   | [ ] Implemented | N/A       |
| `EcucBooleanParamDef`            | [ ] Implemented | N/A       |
| `EcucFloatParamDef`              | [ ] Implemented | N/A       |
| `EcucForeignReferenceDef`        | [ ] Implemented | N/A       |
| `EcucLinkerSymbolDef`            | [ ] Implemented | N/A       |
| `EcucReferenceDef`               | [ ] Implemented | N/A       |
| `EcucSymbolicNameReferenceDef`   | [ ] Implemented | N/A       |
| `EcucUriReferenceDef`            | [ ] Implemented | N/A       |
| `EcucConditionFormula`           | [ ] Implemented | N/A       |
| `EcucParameterDerivationFormula` | [ ] Implemented | N/A       |
| `EcucQueryExpression`            | [ ] Implemented | N/A       |

## Group20

Status: **0/29** completed

| Class Name                         | Status          | Commit ID |
| ---------------------------------- | --------------- | --------- |
| `DoIpLogicAddress`                 | [ ] Implemented | N/A       |
| `DoIpTpConnection`                 | [ ] Implemented | N/A       |
| `CryptoKeySlotTypeEnum`            | [ ] Implemented | N/A       |
| `CryptoObjectTypeEnum`             | [ ] Implemented | N/A       |
| `CryptoKeySlotAllowedModification` | [ ] Implemented | N/A       |
| `CryptoKeySlotContentAllowedUsage` | [ ] Implemented | N/A       |
| `DataLinkLayerRule`                | [ ] Implemented | N/A       |
| `NetworkLayerRule`                 | [ ] Implemented | N/A       |
| `TransportLayerRule`               | [ ] Implemented | N/A       |
| `PayloadBytePatternRule`           | [ ] Implemented | N/A       |
| `SomeipProtocolRule`               | [ ] Implemented | N/A       |
| `SomeipSdRule`                     | [ ] Implemented | N/A       |
| `DoIpRule`                         | [ ] Implemented | N/A       |
| `MemorySection`                    | [ ] Implemented | N/A       |
| `SectionNamePrefix`                | [ ] Implemented | N/A       |
| `HardwareConfiguration`            | [ ] Implemented | N/A       |
| `SoftwareContext`                  | [ ] Implemented | N/A       |
| `SoAdRoutingGroup`                 | [ ] Implemented | N/A       |
| `StackUsage`                       | [ ] Implemented | N/A       |
| `MeasuredStackUsage`               | [ ] Implemented | N/A       |
| `RoughEstimateStackUsage`          | [ ] Implemented | N/A       |
| `WorstCaseStackUsage`              | [ ] Implemented | N/A       |
| `MacAddressString`                 | [ ] Implemented | N/A       |
| `PayloadBytePatternRulePart`       | [ ] Implemented | N/A       |
| `TcpRule`                          | [ ] Implemented | N/A       |
| `IcmpRule`                         | [ ] Implemented | N/A       |
| `Ipv4Rule`                         | [ ] Implemented | N/A       |
| `Ipv6Rule`                         | [ ] Implemented | N/A       |
| `UdpRule`                          | [ ] Implemented | N/A       |

## Group21

Status: **0/55** completed

| Class Name                           | Status          | Commit ID |
| ------------------------------------ | --------------- | --------- |
| `Identifier`                         | [ ] Implemented | N/A       |
| `LLongName`                          | [ ] Implemented | N/A       |
| `MixedContentForLongName`            | [ ] Implemented | N/A       |
| `Referrable`                         | [ ] Implemented | N/A       |
| `ReferrableSubtypesEnum`             | [ ] Implemented | N/A       |
| `SdgDef`                             | [ ] Implemented | N/A       |
| `SdgElementWithGid`                  | [ ] Implemented | N/A       |
| `SdgClass`                           | [ ] Implemented | N/A       |
| `SdgAttribute`                       | [ ] Implemented | N/A       |
| `SdgAbstractPrimitiveAttribute`      | [ ] Implemented | N/A       |
| `SdgPrimitiveAttribute`              | [ ] Implemented | N/A       |
| `SdgPrimitiveAttributeWithVariation` | [ ] Implemented | N/A       |
| `SdgAggregationWithVariation`        | [ ] Implemented | N/A       |
| `SdgReference`                       | [ ] Implemented | N/A       |
| `SdgAbstractForeignReference`        | [ ] Implemented | N/A       |
| `SdgForeignReference`                | [ ] Implemented | N/A       |
| `SdgForeignReferenceWithVariation`   | [ ] Implemented | N/A       |
| `AbstractValueRestriction`           | [ ] Implemented | N/A       |
| `AbstractVariationRestriction`       | [ ] Implemented | N/A       |
| `FullBindingTimeEnum`                | [ ] Implemented | N/A       |
| `CIdentifier`                        | [ ] Implemented | N/A       |
| `CategoryString`                     | [ ] Implemented | N/A       |
| `DateTime`                           | [ ] Implemented | N/A       |
| `DiagRequirementIdString`            | [ ] Implemented | N/A       |
| `Ip4AddressString`                   | [ ] Implemented | N/A       |
| `Ip6AddressString`                   | [ ] Implemented | N/A       |
| `McdIdentifier`                      | [ ] Implemented | N/A       |
| `RegularExpression`                  | [ ] Implemented | N/A       |
| `RevisionLabelString`                | [ ] Created     | N/A       |
| `SymbolString`                       | [ ] Implemented | N/A       |
| `UriString`                          | [ ] Implemented | N/A       |
| `VerbatimStringPlain`                | [ ] Implemented | N/A       |
| `CseCodeType`                        | [ ] Implemented | N/A       |
| `EvaluatedVariantSet`                | [ ] Implemented | N/A       |
| `PredefinedVariant`                  | [ ] Implemented | N/A       |
| `DocumentationBlock`                 | [ ] Implemented | N/A       |
| `MultiLanguageVerbatim`              | [ ] Implemented | N/A       |
| `List`                               | [ ] Pending     | N/A       |
| `LabeledList`                        | [ ] Implemented | N/A       |
| `LabeledItem`                        | [ ] Implemented | N/A       |
| `IndentSample`                       | [ ] Implemented | N/A       |
| `ItemLabelPosEnum`                   | [ ] Implemented | N/A       |
| `DefList`                            | [ ] Implemented | N/A       |
| `DefItem`                            | [ ] Implemented | N/A       |
| `MlFormula`                          | [ ] Implemented | N/A       |
| `Note`                               | [ ] Implemented | N/A       |
| `NoteTypeEnum`                       | [ ] Implemented | N/A       |
| `Traceable`                          | [ ] Implemented | N/A       |
| `EmphasisText`                       | [ ] Implemented | N/A       |
| `IndexEntry`                         | [ ] Implemented | N/A       |
| `Superscript`                        | [ ] Implemented | N/A       |
| `Tt`                                 | [ ] Implemented | N/A       |
| `EEnumFont`                          | [ ] Implemented | N/A       |
| `EEnum`                              | [ ] Implemented | N/A       |
| `DocumentationContext`               | [ ] Implemented | N/A       |

## Group22

Status: **0/75** completed

| Class Name                             | Status          | Commit ID |
| -------------------------------------- | --------------- | --------- |
| `AnyInstanceRef`                       | [ ] Implemented | N/A       |
| `Chapter`                              | [ ] Implemented | N/A       |
| `PredefinedChapter`                    | [ ] Implemented | N/A       |
| `FloatEnum`                            | [ ] Implemented | N/A       |
| `Topic1`                               | [ ] Implemented | N/A       |
| `TopicOrMsrQuery`                      | [ ] Implemented | N/A       |
| `ChapterOrMsrQuery`                    | [ ] Implemented | N/A       |
| `MsrQueryProps`                        | [ ] Implemented | N/A       |
| `MsrQueryArg`                          | [ ] Implemented | N/A       |
| `MultiLanguageOverviewParagraph`       | [ ] Implemented | N/A       |
| `PgwideEnum`                           | [ ] Implemented | N/A       |
| `MultiLanguagePlainText`               | [ ] Implemented | N/A       |
| `LanguageSpecific`                     | [ ] Implemented | N/A       |
| `LEnum`                                | [ ] Implemented | N/A       |
| `AclPermission`                        | [ ] Implemented | N/A       |
| `AclObjectSet`                         | [ ] Implemented | N/A       |
| `AclOperation`                         | [ ] Implemented | N/A       |
| `AclRole`                              | [ ] Implemented | N/A       |
| `AclScopeEnum`                         | [ ] Implemented | N/A       |
| `LifeCycleStateDefinitionGroup`        | [ ] Implemented | N/A       |
| `LifeCycleState`                       | [ ] Created     | N/A       |
| `ViewMapSet`                           | [ ] Implemented | N/A       |
| `ViewMap`                              | [ ] Implemented | N/A       |
| `BswModuleDescription`                 | [ ] Implemented | N/A       |
| `BswModuleEntry`                       | [ ] Implemented | N/A       |
| `BswEntryKindEnum`                     | [ ] Implemented | N/A       |
| `BswExecutionContext`                  | [ ] Implemented | N/A       |
| `BswCallType`                          | [ ] Implemented | N/A       |
| `SwServiceImplPolicyEnum`              | [ ] Implemented | N/A       |
| `SwServiceArg`                         | [ ] Implemented | N/A       |
| `SwPointerTargetProps`                 | [ ] Implemented | N/A       |
| `ArgumentDirectionEnum`                | [ ] Implemented | N/A       |
| `ModeDeclarationGroup`                 | [ ] Implemented | N/A       |
| `ModeDeclaration`                      | [ ] Implemented | N/A       |
| `ModeTransition`                       | [ ] Implemented | N/A       |
| `ModeErrorBehavior`                    | [ ] Implemented | N/A       |
| `ModeErrorReactionPolicyEnum`          | [ ] Implemented | N/A       |
| `AccessCountSet`                       | [ ] Implemented | N/A       |
| `AccessCount`                          | [ ] Implemented | N/A       |
| `AbstractAccessPoint`                  | [ ] Implemented | N/A       |
| `InternalBehavior`                     | [ ] Implemented | N/A       |
| `ExecutableEntity`                     | [ ] Implemented | N/A       |
| `BswModuleEntity`                      | [ ] Implemented | N/A       |
| `BswCalledEntity`                      | [ ] Implemented | N/A       |
| `BswSchedulableEntity`                 | [ ] Implemented | N/A       |
| `BswInterruptCategory`                 | [ ] Implemented | N/A       |
| `BswAsynchronousServerCallPoint`       | [ ] Implemented | N/A       |
| `BswAsynchronousServerCallResultPoint` | [ ] Implemented | N/A       |
| `BswVariableAccess`                    | [ ] Implemented | N/A       |
| `ExclusiveArea`                        | [ ] Implemented | N/A       |
| `BswExclusiveAreaPolicy`               | [ ] Implemented | N/A       |
| `ExclusiveAreaNestingOrder`            | [ ] Implemented | N/A       |
| `BswSchedulerNamePrefix`               | [ ] Implemented | N/A       |
| `BswEvent`                             | [ ] Implemented | N/A       |
| `BswScheduleEvent`                     | [ ] Implemented | N/A       |
| `BswInterruptEvent`                    | [ ] Implemented | N/A       |
| `BswBackgroundEvent`                   | [ ] Implemented | N/A       |
| `BswOsTaskExecutionEvent`              | [ ] Implemented | N/A       |
| `BswExternalTriggerOccurredEvent`      | [ ] Implemented | N/A       |
| `BswModeSwitchEvent`                   | [ ] Implemented | N/A       |
| `BswOperationInvokedEvent`             | [ ] Implemented | N/A       |
| `BswTriggerDirectImplementation`       | [ ] Implemented | N/A       |
| `BswModeSenderPolicy`                  | [ ] Implemented | N/A       |
| `BswModeReceiverPolicy`                | [ ] Implemented | N/A       |
| `ParameterDataPrototype`               | [ ] Implemented | N/A       |
| `BswDistinguishedPartition`            | [ ] Implemented | N/A       |
| `BswImplementation`                    | [ ] Implemented | N/A       |
| `AutosarEngineeringObject`             | [ ] Implemented | N/A       |
| `AlignmentType`                        | [ ] Implemented | N/A       |
| `SwAddrMethod`                         | [ ] Implemented | N/A       |
| `MemoryAllocationKeywordPolicyType`    | [ ] Implemented | N/A       |
| `SectionInitializationPolicyType`      | [ ] Implemented | N/A       |
| `MemorySectionType`                    | [ ] Implemented | N/A       |
| `HeapUsage`                            | [ ] Implemented | N/A       |
| `WorstCaseHeapUsage`                   | [ ] Implemented | N/A       |

## Group23

Status: **0/75** completed

| Class Name                                        | Status          | Commit ID |
| ------------------------------------------------- | --------------- | --------- |
| `MeasuredHeapUsage`                               | [ ] Implemented | N/A       |
| `RoughEstimateHeapUsage`                          | [ ] Implemented | N/A       |
| `ExecutionTime`                                   | [ ] Implemented | N/A       |
| `MemorySectionLocation`                           | [ ] Implemented | N/A       |
| `AnalyzedExecutionTime`                           | [ ] Implemented | N/A       |
| `MeasuredExecutionTime`                           | [ ] Implemented | N/A       |
| `SimulatedExecutionTime`                          | [ ] Implemented | N/A       |
| `RoughEstimateOfExecutionTime`                    | [ ] Implemented | N/A       |
| `McSupportData`                                   | [ ] Implemented | N/A       |
| `AliasNameSet`                                    | [ ] Implemented | N/A       |
| `AliasNameAssignment`                             | [ ] Implemented | N/A       |
| `McDataInstance`                                  | [ ] Implemented | N/A       |
| `McSwEmulationMethodSupport`                      | [ ] Implemented | N/A       |
| `McParameterElementGroup`                         | [ ] Implemented | N/A       |
| `ImplementationElementInParameterInstanceRef`     | [ ] Implemented | N/A       |
| `McFunction`                                      | [ ] Implemented | N/A       |
| `McFunctionDataRefSet`                            | [ ] Implemented | N/A       |
| `McGroup`                                         | [ ] Implemented | N/A       |
| `McGroupDataRefSet`                               | [ ] Implemented | N/A       |
| `McDataAccessDetails`                             | [ ] Implemented | N/A       |
| `RptSupportData`                                  | [ ] Implemented | N/A       |
| `RptSwPrototypingAccess`                          | [ ] Implemented | N/A       |
| `RptComponent`                                    | [ ] Implemented | N/A       |
| `RptExecutableEntity`                             | [ ] Implemented | N/A       |
| `RptExecutableEntityEvent`                        | [ ] Implemented | N/A       |
| `RptImplPolicy`                                   | [ ] Implemented | N/A       |
| `RptEnablerImplTypeEnum`                          | [ ] Implemented | N/A       |
| `RptPreparationEnum`                              | [ ] Implemented | N/A       |
| `RptExecutableEntityProperties`                   | [ ] Implemented | N/A       |
| `RptExecutionControlEnum`                         | [ ] Implemented | N/A       |
| `RptServicePointEnum`                             | [ ] Implemented | N/A       |
| `RptExecutionContext`                             | [ ] Implemented | N/A       |
| `RptAccessEnum`                                   | [ ] Implemented | N/A       |
| `RptServicePoint`                                 | [ ] Implemented | N/A       |
| `ServiceDependency`                               | [ ] Implemented | N/A       |
| `BswServiceDependency`                            | [ ] Implemented | N/A       |
| `RoleBasedBswModuleEntryAssignment`               | [ ] Implemented | N/A       |
| `RoleBasedDataTypeAssignment`                     | [ ] Implemented | N/A       |
| `MaxCommModeEnum`                                 | [ ] Implemented | N/A       |
| `SupervisedEntityNeeds`                           | [ ] Implemented | N/A       |
| `ComMgrUserNeeds`                                 | [ ] Implemented | N/A       |
| `DoIpServiceNeeds`                                | [ ] Implemented | N/A       |
| `DiagnosticIoControlNeeds`                        | [ ] Implemented | N/A       |
| `DiagnosticEventNeeds`                            | [ ] Implemented | N/A       |
| `DiagEventDebounceTimeBased`                      | [ ] Implemented | N/A       |
| `ErrorTracerNeeds`                                | [ ] Implemented | N/A       |
| `TracedFailure`                                   | [ ] Implemented | N/A       |
| `DevelopmentError`                                | [ ] Implemented | N/A       |
| `RuntimeError`                                    | [ ] Implemented | N/A       |
| `DiagnosticDataIdentifier`                        | [ ] Implemented | N/A       |
| `DiagnosticDynamicDataIdentifier`                 | [ ] Implemented | N/A       |
| `DiagnosticAbstractDataIdentifier`                | [ ] Implemented | N/A       |
| `DiagnosticParameter`                             | [ ] Implemented | N/A       |
| `DiagnosticParameterElement`                      | [ ] Implemented | N/A       |
| `DiagnosticParameterIdent`                        | [ ] Implemented | N/A       |
| `DiagnosticAbstractParameter`                     | [ ] Implemented | N/A       |
| `DiagnosticDataElement`                           | [ ] Implemented | N/A       |
| `DiagnosticContributionSet`                       | [ ] Implemented | N/A       |
| `DiagnosticProtocol`                              | [ ] Implemented | N/A       |
| `TpConnectionIdent`                               | [ ] Implemented | N/A       |
| `DiagnosticCommonProps`                           | [ ] Implemented | N/A       |
| `DiagnosticOccurrenceCounterProcessingEnum`       | [ ] Implemented | N/A       |
| `DiagnosticTypeOfDtcSupportedEnum`                | [ ] Implemented | N/A       |
| `DiagnosticEventCombinationBehaviorEnum`          | [ ] Implemented | N/A       |
| `DiagnosticEventCombinationReportingBehaviorEnum` | [ ] Implemented | N/A       |
| `DiagnosticCustomServiceInstance`                 | [ ] Implemented | N/A       |
| `DiagnosticCustomServiceClass`                    | [ ] Implemented | N/A       |
| `DiagnosticAuthRole`                              | [ ] Implemented | N/A       |
| `DiagnosticCompareTypeEnum`                       | [ ] Implemented | N/A       |
| `DiagnosticEnvDataCondition`                      | [ ] Implemented | N/A       |
| `DiagnosticEnvDataElementCondition`               | [ ] Implemented | N/A       |
| `DiagnosticEnvModeCondition`                      | [ ] Created     | N/A       |
| `DiagnosticEnvSwcModeElement`                     | [ ] Implemented | N/A       |
| `DiagnosticEnvBswModeElement`                     | [ ] Implemented | N/A       |
| `DiagnosticSessionControl`                        | [ ] Implemented | N/A       |

## Group24

Status: **0/75** completed

| Class Name                                                 | Status          | Commit ID |
| ---------------------------------------------------------- | --------------- | --------- |
| `DiagnosticSessionControlClass`                            | [ ] Implemented | N/A       |
| `DiagnosticSecurityAccess`                                 | [ ] Implemented | N/A       |
| `DiagnosticSecurityAccessClass`                            | [ ] Implemented | N/A       |
| `DiagnosticAuthentication`                                 | [ ] Implemented | N/A       |
| `DiagnosticAuthenticationClass`                            | [ ] Implemented | N/A       |
| `DiagnosticAuthenticationConfiguration`                    | [ ] Implemented | N/A       |
| `DiagnosticVerifyCertificateBidirectional`                 | [ ] Implemented | N/A       |
| `DiagnosticVerifyCertificateUnidirectional`                | [ ] Implemented | N/A       |
| `DiagnosticDeAuthentication`                               | [ ] Implemented | N/A       |
| `DiagnosticProofOfOwnership`                               | [ ] Implemented | N/A       |
| `DiagnosticAuthTransmitCertificate`                        | [ ] Implemented | N/A       |
| `DiagnosticAuthTransmitCertificateEvaluation`              | [ ] Implemented | N/A       |
| `DiagnosticEcuReset`                                       | [ ] Implemented | N/A       |
| `DiagnosticEcuResetClass`                                  | [ ] Implemented | N/A       |
| `DiagnosticResponseToEcuResetEnum`                         | [ ] Implemented | N/A       |
| `CommunicationCluster`                                     | [ ] Implemented | N/A       |
| `DiagnosticComControl`                                     | [ ] Implemented | N/A       |
| `DiagnosticComControlSpecificChannel`                      | [ ] Implemented | N/A       |
| `DiagnosticComControlClass`                                | [ ] Created     | N/A       |
| `DiagnosticComControlSubNodeChannel`                       | [ ] Created     | N/A       |
| `DiagnosticControlDTCSetting`                              | [ ] Created     | N/A       |
| `DiagnosticControlDTCSettingClass`                         | [ ] Created     | N/A       |
| `DiagnosticReadDataByIdentifier`                           | [ ] Created     | N/A       |
| `DiagnosticWriteDataByIdentifier`                          | [ ] Created     | N/A       |
| `DiagnosticWriteDataByIdentifierClass`                     | [ ] Created     | N/A       |
| `DiagnosticDataByIdentifier`                               | [ ] Created     | N/A       |
| `DiagnosticReadDataByIdentifierClass`                      | [ ] Created     | N/A       |
| `DiagnosticReadScalingDataByIdentifier`                    | [ ] Created     | N/A       |
| `DiagnosticReadScalingDataByIdentifierClass`               | [ ] Created     | N/A       |
| `DiagnosticIOControl`                                      | [ ] Created     | N/A       |
| `DiagnosticIoControlClass`                                 | [ ] Created     | N/A       |
| `DiagnosticControlEnableMaskBit`                           | [ ] Created     | N/A       |
| `DiagnosticRoutineSubfunction`                             | [ ] Created     | N/A       |
| `DiagnosticRoutine`                                        | [ ] Created     | N/A       |
| `DiagnosticStartRoutine`                                   | [ ] Created     | N/A       |
| `DiagnosticStopRoutine`                                    | [ ] Created     | N/A       |
| `DiagnosticRequestRoutineResults`                          | [ ] Created     | N/A       |
| `DiagnosticRoutineControl`                                 | [ ] Created     | N/A       |
| `DiagnosticRoutineControlClass`                            | [ ] Created     | N/A       |
| `DiagnosticDynamicallyDefineDataIdentifier`                | [ ] Created     | N/A       |
| `DiagnosticDynamicallyDefineDataIdentifierClass`           | [ ] Created     | N/A       |
| `DiagnosticHandleDDDIConfigurationEnum`                    | [ ] Created     | N/A       |
| `DiagnosticDynamicallyDefineDataIdentifierSubfunctionEnum` | [ ] Created     | N/A       |
| `DiagnosticReadDataByPeriodicID`                           | [ ] Created     | N/A       |
| `DiagnosticReadDataByPeriodicIDClass`                      | [ ] Created     | N/A       |
| `DiagnosticPeriodicRate`                                   | [ ] Created     | N/A       |
| `DiagnosticPeriodicRateCategoryEnum`                       | [ ] Created     | N/A       |
| `DiagnosticResponseOnEvent`                                | [ ] Created     | N/A       |
| `DiagnosticResponseOnEventClass`                           | [ ] Created     | N/A       |
| `DiagnosticEventWindow`                                    | [ ] Created     | N/A       |
| `DiagnosticEventWindowTimeEnum`                            | [ ] Created     | N/A       |
| `DiagnosticResponseOnEventActionEnum`                      | [ ] Created     | N/A       |
| `DiagnosticReadDTCInformation`                             | [ ] Created     | N/A       |
| `DiagnosticReadDTCInformationClass`                        | [ ] Created     | N/A       |
| `DiagnosticClearDiagnosticInformation`                     | [ ] Created     | N/A       |
| `DiagnosticClearDiagnosticInformationClass`                | [ ] Created     | N/A       |
| `DiagnosticMemoryByAddress`                                | [ ] Created     | N/A       |
| `DiagnosticMemoryAddressableRangeAccess`                   | [ ] Created     | N/A       |
| `DiagnosticMemoryIdentifier`                               | [ ] Created     | N/A       |
| `DiagnosticWriteMemoryByAddress`                           | [ ] Created     | N/A       |
| `DiagnosticWriteMemoryByAddressClass`                      | [ ] Created     | N/A       |
| `DiagnosticReadMemoryByAddress`                            | [ ] Created     | N/A       |
| `DiagnosticReadMemoryByAddressClass`                       | [ ] Created     | N/A       |
| `DiagnosticTransferExit`                                   | [ ] Created     | N/A       |
| `DiagnosticTransferExitClass`                              | [ ] Created     | N/A       |
| `DiagnosticDataTransfer`                                   | [ ] Created     | N/A       |
| `DiagnosticDataTransferClass`                              | [ ] Created     | N/A       |
| `DiagnosticRequestDownload`                                | [ ] Created     | N/A       |
| `DiagnosticRequestDownloadClass`                           | [ ] Created     | N/A       |
| `DiagnosticRequestUpload`                                  | [ ] Created     | N/A       |
| `DiagnosticRequestUploadClass`                             | [ ] Created     | N/A       |
| `DiagnosticRequestFileTransfer`                            | [ ] Created     | N/A       |
| `DiagnosticRequestFileTransferClass`                       | [ ] Created     | N/A       |
| `DiagnosticParameterIdentifier`                            | [ ] Created     | N/A       |
| `DiagnosticParameterSupportInfo`                           | [ ] Created     | N/A       |

## Group25

Status: **0/75** completed

| Class Name                                                | Status      | Commit ID |
| --------------------------------------------------------- | ----------- | --------- |
| `DiagnosticSupportInfoByte`                               | [ ] Created | N/A       |
| `DiagnosticRequestCurrentPowertrainData`                  | [ ] Created | N/A       |
| `DiagnosticRequestCurrentPowertrainDataClass`             | [ ] Created | N/A       |
| `DiagnosticRequestPowertrainFreezeFrameData`              | [ ] Created | N/A       |
| `DiagnosticRequestPowertrainFreezeFrameDataClass`         | [ ] Created | N/A       |
| `DiagnosticPowertrainFreezeFrame`                         | [ ] Created | N/A       |
| `DiagnosticRequestEmissionRelatedDTC`                     | [ ] Created | N/A       |
| `DiagnosticRequestEmissionRelatedDTCClass`                | [ ] Created | N/A       |
| `DiagnosticClearResetEmissionRelatedInfo`                 | [ ] Created | N/A       |
| `DiagnosticClearResetEmissionRelatedInfoClass`            | [ ] Created | N/A       |
| `DiagnosticRequestOnBoardMonitoringTestResults`           | [ ] Created | N/A       |
| `DiagnosticRequestOnBoardMonitoringTestResultsClass`      | [ ] Created | N/A       |
| `DiagnosticRequestControlOfOnBoardDevice`                 | [ ] Created | N/A       |
| `DiagnosticRequestControlOfOnBoardDeviceClass`            | [ ] Created | N/A       |
| `DiagnosticTestRoutineIdentifier`                         | [ ] Created | N/A       |
| `DiagnosticRequestVehicleInfo`                            | [ ] Created | N/A       |
| `DiagnosticRequestVehicleInfoClass`                       | [ ] Created | N/A       |
| `DiagnosticInfoType`                                      | [ ] Created | N/A       |
| `DiagnosticRequestEmissionRelatedDTCPermanentStatus`      | [ ] Created | N/A       |
| `DiagnosticRequestEmissionRelatedDTCPermanentStatusClass` | [ ] Created | N/A       |
| `DiagnosticEvent`                                         | [ ] Created | N/A       |
| `DiagnosticClearEventAllowedBehaviorEnum`                 | [ ] Created | N/A       |
| `DiagnosticConnectedIndicator`                            | [ ] Created | N/A       |
| `DiagnosticEventClearAllowedEnum`                         | [ ] Created | N/A       |
| `DiagnosticEventKindEnum`                                 | [ ] Created | N/A       |
| `DiagnosticConnectedIndicatorBehaviorEnum`                | [ ] Created | N/A       |
| `DiagnosticTroubleCodeUds`                                | [ ] Created | N/A       |
| `DiagnosticTroubleCodeObd`                                | [ ] Created | N/A       |
| `EventObdReadinessGroup`                                  | [ ] Created | N/A       |
| `DiagnosticTroubleCode`                                   | [ ] Created | N/A       |
| `DiagnosticTroubleCodeGroup`                              | [ ] Created | N/A       |
| `DiagnosticMemoryDestination`                             | [ ] Created | N/A       |
| `DiagnosticMemoryEntryStorageTriggerEnum`                 | [ ] Created | N/A       |
| `DiagnosticClearDtcLimitationEnum`                        | [ ] Created | N/A       |
| `DiagnosticEventDisplacementStrategyEnum`                 | [ ] Created | N/A       |
| `DiagnosticStatusBitHandlingTestFailedSinceLastClearEnum` | [ ] Created | N/A       |
| `DiagnosticTypeOfFreezeFrameRecordNumerationEnum`         | [ ] Created | N/A       |
| `DiagnosticMemoryDestinationPrimary`                      | [ ] Created | N/A       |
| `DiagnosticMemoryDestinationUserDefined`                  | [ ] Created | N/A       |
| `DiagnosticTroubleCodeProps`                              | [ ] Created | N/A       |
| `DiagnosticSignificanceEnum`                              | [ ] Created | N/A       |
| `DiagnosticUdsSeverityEnum`                               | [ ] Created | N/A       |
| `DiagnosticDataIdentifierSet`                             | [ ] Created | N/A       |
| `DiagnosticWwhObdDtcClassEnum`                            | [ ] Created | N/A       |
| `DiagnosticTroubleCodeUdsToTroubleCodeObdMapping`         | [ ] Created | N/A       |
| `DiagnosticExtendedDataRecord`                            | [ ] Created | N/A       |
| `DiagnosticRecordTriggerEnum`                             | [ ] Created | N/A       |
| `DiagnosticFreezeFrame`                                   | [ ] Created | N/A       |
| `DiagnosticCondition`                                     | [ ] Created | N/A       |
| `DiagnosticEnableCondition`                               | [ ] Created | N/A       |
| `DiagnosticStorageCondition`                              | [ ] Created | N/A       |
| `DiagnosticDebounceAlgorithmProps`                        | [ ] Created | N/A       |
| `DiagnosticDebounceBehaviorEnum`                          | [ ] Created | N/A       |
| `DiagnosticConditionGroup`                                | [ ] Created | N/A       |
| `DiagnosticEnableConditionGroup`                          | [ ] Created | N/A       |
| `DiagnosticStorageConditionGroup`                         | [ ] Created | N/A       |
| `DiagnosticOperationCycle`                                | [ ] Created | N/A       |
| `DiagnosticOperationCycleTypeEnum`                        | [ ] Created | N/A       |
| `DiagnosticAging`                                         | [ ] Created | N/A       |
| `DiagnosticIndicator`                                     | [ ] Created | N/A       |
| `DiagnosticTestResultUpdateEnum`                          | [ ] Created | N/A       |
| `DiagnosticTestIdentifier`                                | [ ] Created | N/A       |
| `DiagnosticMeasurementIdentifier`                         | [ ] Created | N/A       |
| `DiagnosticEcuInstanceProps`                              | [ ] Created | N/A       |
| `DiagnosticObdSupportEnum`                                | [ ] Created | N/A       |
| `DiagnosticIumpr`                                         | [ ] Created | N/A       |
| `DiagnosticIumprKindEnum`                                 | [ ] Created | N/A       |
| `DiagnosticIumprGroup`                                    | [ ] Created | N/A       |
| `DiagnosticIumprGroupIdentifier`                          | [ ] Created | N/A       |
| `DiagnosticIumprDenominatorGroup`                         | [ ] Created | N/A       |
| `DiagnosticFimAliasEvent`                                 | [ ] Created | N/A       |
| `DiagnosticAbstractAliasEvent`                            | [ ] Created | N/A       |
| `DiagnosticFunctionIdentifier`                            | [ ] Created | N/A       |
| `DiagnosticFunctionIdentifierInhibit`                     | [ ] Created | N/A       |
| `DiagnosticFunctionInhibitSource`                         | [ ] Created | N/A       |

## Group26

Status: **0/75** completed

| Class Name                                      | Status          | Commit ID |
| ----------------------------------------------- | --------------- | --------- |
| `DiagnosticInhibitionMaskEnum`                  | [ ] Created     | N/A       |
| `DiagnosticFimEventGroup`                       | [ ] Created     | N/A       |
| `DiagnosticJ1939Spn`                            | [ ] Created     | N/A       |
| `DiagnosticJ1939FreezeFrame`                    | [ ] Created     | N/A       |
| `DiagnosticJ1939ExpandedFreezeFrame`            | [ ] Created     | N/A       |
| `DiagnosticTroubleCodeJ1939DtcKindEnum`         | [ ] Created     | N/A       |
| `DiagnosticTroubleCodeJ1939`                    | [ ] Created     | N/A       |
| `DiagnosticMapping`                             | [ ] Implemented | N/A       |
| `DiagnosticServiceDataMapping`                  | [ ] Created     | N/A       |
| `DiagnosticParameterElementAccess`              | [ ] Created     | N/A       |
| `DiagnosticServiceMappingDiagTarget`            | [ ] Created     | N/A       |
| `DiagnosticSwMapping`                           | [ ] Created     | N/A       |
| `DiagnosticServiceSwMapping`                    | [ ] Created     | N/A       |
| `BswServiceDependencyIdent`                     | [ ] Implemented | N/A       |
| `DiagnosticAuthTransmitCertificateMapping`      | [ ] Created     | N/A       |
| `DiagnosticSecurityEventReportingModeMapping`   | [ ] Created     | N/A       |
| `DiagnosticEventToTroubleCodeUdsMapping`        | [ ] Created     | N/A       |
| `DiagnosticEventToOperationCycleMapping`        | [ ] Created     | N/A       |
| `DiagnosticEventToDebounceAlgorithmMapping`     | [ ] Created     | N/A       |
| `DiagnosticEventToEnableConditionGroupMapping`  | [ ] Created     | N/A       |
| `DiagnosticEventToStorageConditionGroupMapping` | [ ] Created     | N/A       |
| `DiagnosticEventPortMapping`                    | [ ] Created     | N/A       |
| `DiagnosticOperationCyclePortMapping`           | [ ] Created     | N/A       |
| `DiagnosticEnableConditionPortMapping`          | [ ] Created     | N/A       |
| `DiagnosticStorageConditionPortMapping`         | [ ] Created     | N/A       |
| `DiagnosticDemProvidedDataMapping`              | [ ] Created     | N/A       |
| `DiagnosticMasterToSlaveEventMapping`           | [ ] Created     | N/A       |
| `DiagnosticEventToSecurityEventMapping`         | [ ] Created     | N/A       |
| `DiagnosticInhibitSourceEventMapping`           | [ ] Created     | N/A       |
| `DiagnosticFimAliasEventMapping`                | [ ] Created     | N/A       |
| `DiagnosticFimAliasEventGroup`                  | [ ] Created     | N/A       |
| `DiagnosticFimAliasEventGroupMapping`           | [ ] Created     | N/A       |
| `DiagnosticFimFunctionMapping`                  | [ ] Created     | N/A       |
| `DiagnosticIumprToFunctionIdentifierMapping`    | [ ] Created     | N/A       |
| `DiagnosticJ1939SpnMapping`                     | [ ] Created     | N/A       |
| `DiagnosticJ1939Node`                           | [ ] Created     | N/A       |
| `DiagnosticJ1939SwMapping`                      | [ ] Created     | N/A       |
| `DiagnosticEventToTroubleCodeJ1939Mapping`      | [ ] Created     | N/A       |
| `CpSoftwareClusterResource`                     | [ ] Created     | N/A       |
| `RoleBasedResourceDependency`                   | [ ] Created     | N/A       |
| `CpSwClusterToDiagEventMapping`                 | [ ] Created     | N/A       |
| `CpSwClusterResourceToDiagDataElemMapping`      | [ ] Created     | N/A       |
| `CpSwClusterToDiagRoutineSubfunctionMapping`    | [ ] Created     | N/A       |
| `CpSwClusterResourceToDiagFunctionIdMapping`    | [ ] Created     | N/A       |
| `EcucDefinitionCollection`                      | [ ] Implemented | N/A       |
| `EcucModuleDef`                                 | [ ] Implemented | N/A       |
| `EcucContainerDef`                              | [ ] Implemented | N/A       |
| `EcucParamConfContainerDef`                     | [ ] Implemented | N/A       |
| `EcucChoiceContainerDef`                        | [ ] Implemented | N/A       |
| `EcucDefinitionElement`                         | [ ] Implemented | N/A       |
| `EcucCommonAttributes`                          | [ ] Implemented | N/A       |
| `EcucAbstractConfigurationClass`                | [ ] Implemented | N/A       |
| `EcucValueConfigurationClass`                   | [ ] Implemented | N/A       |
| `EcucMultiplicityConfigurationClass`            | [ ] Implemented | N/A       |
| `EcucConfigurationVariantEnum`                  | [ ] Implemented | N/A       |
| `EcucParameterDef`                              | [ ] Implemented | N/A       |
| `EcucIntegerParamDef`                           | [ ] Implemented | N/A       |
| `EcucAbstractStringParamDef`                    | [ ] Implemented | N/A       |
| `EcucStringParamDef`                            | [ ] Implemented | N/A       |
| `EcucMultilineStringParamDef`                   | [ ] Implemented | N/A       |
| `EcucFunctionNameDef`                           | [ ] Implemented | N/A       |
| `EcucEnumerationParamDef`                       | [ ] Implemented | N/A       |
| `EcucEnumerationLiteralDef`                     | [ ] Implemented | N/A       |
| `EcucAddInfoParamDef`                           | [ ] Implemented | N/A       |
| `EcucAbstractReferenceDef`                      | [ ] Implemented | N/A       |
| `EcucAbstractInternalReferenceDef`              | [ ] Implemented | N/A       |
| `EcucAbstractExternalReferenceDef`              | [ ] Implemented | N/A       |
| `EcucChoiceReferenceDef`                        | [ ] Implemented | N/A       |
| `EcucInstanceReferenceDef`                      | [ ] Implemented | N/A       |
| `EcucDestinationUriDefSet`                      | [ ] Implemented | N/A       |
| `EcucDestinationUriDef`                         | [ ] Implemented | N/A       |
| `EcucDestinationUriPolicy`                      | [ ] Implemented | N/A       |
| `EcucDestinationUriNestingContractEnum`         | [ ] Implemented | N/A       |
| `EcucDerivationSpecification`                   | [ ] Implemented | N/A       |
| `EcucQuery`                                     | [ ] Implemented | N/A       |

## Group27

Status: **0/75** completed

| Class Name                                  | Status          | Commit ID |
| ------------------------------------------- | --------------- | --------- |
| `EcucConditionSpecification`                | [ ] Implemented | N/A       |
| `EcucValidationCondition`                   | [ ] Implemented | N/A       |
| `EcucIndexableValue`                        | [ ] Implemented | N/A       |
| `EcucModuleConfigurationValues`             | [ ] Implemented | N/A       |
| `EcucContainerValue`                        | [ ] Implemented | N/A       |
| `EcucParameterValue`                        | [ ] Implemented | N/A       |
| `EcucTextualParamValue`                     | [ ] Implemented | N/A       |
| `EcucNumericalParamValue`                   | [ ] Implemented | N/A       |
| `EcucAddInfoParamValue`                     | [ ] Implemented | N/A       |
| `EcucAbstractReferenceValue`                | [ ] Implemented | N/A       |
| `EcucReferenceValue`                        | [ ] Implemented | N/A       |
| `EcucInstanceReferenceValue`                | [ ] Implemented | N/A       |
| `HwDescriptionEntity`                       | [ ] Implemented | N/A       |
| `HwPinGroupContent`                         | [ ] Implemented | N/A       |
| `HwElementConnector`                        | [ ] Implemented | N/A       |
| `HwPinGroupConnector`                       | [ ] Implemented | N/A       |
| `HwPinConnector`                            | [ ] Implemented | N/A       |
| `CommunicationController`                   | [ ] Implemented | N/A       |
| `ParameterSwComponentType`                  | [ ] Created     | N/A       |
| `SwComponentType`                           | [ ] Implemented | N/A       |
| `AtomicSwComponentType`                     | [ ] Implemented | N/A       |
| `ApplicationSwComponentType`                | [ ] Implemented | N/A       |
| `SwConnector`                               | [ ] Implemented | N/A       |
| `PassThroughSwConnector`                    | [ ] Implemented | N/A       |
| `InstantiationTimingEventProps`             | [ ] Implemented | N/A       |
| `InstantiationRTEEventProps`                | [ ] Implemented | N/A       |
| `PortInterface`                             | [ ] Implemented | N/A       |
| `ServiceProviderEnum`                       | [ ] Implemented | N/A       |
| `ClientServerInterface`                     | [ ] Implemented | N/A       |
| `ClientServerOperation`                     | [ ] Implemented | N/A       |
| `ArgumentDataPrototype`                     | [ ] Implemented | N/A       |
| `ServerArgumentImplPolicyEnum`              | [ ] Implemented | N/A       |
| `ApplicationError`                          | [ ] Implemented | N/A       |
| `ModeSwitchInterface`                       | [ ] Implemented | N/A       |
| `DataPrototypeMapping`                      | [ ] Implemented | N/A       |
| `ModeDeclarationMapping`                    | [ ] Implemented | N/A       |
| `ImplementationDataTypeSubElementRef`       | [ ] Created     | N/A       |
| `ApplicationCompositeDataTypeSubElementRef` | [ ] Implemented | N/A       |
| `MappingDirectionEnum`                      | [ ] Implemented | N/A       |
| `TextTableValuePair`                        | [ ] Implemented | N/A       |
| `DataTransformation`                        | [ ] Implemented | N/A       |
| `DataTransformationKindEnum`                | [ ] Implemented | N/A       |
| `SenderReceiverAnnotation`                  | [ ] Implemented | N/A       |
| `SenderAnnotation`                          | [ ] Created     | N/A       |
| `ReceiverAnnotation`                        | [ ] Created     | N/A       |
| `ProcessingKindEnum`                        | [ ] Implemented | N/A       |
| `DataLimitKindEnum`                         | [ ] Implemented | N/A       |
| `ClientServerAnnotation`                    | [ ] Implemented | N/A       |
| `IoHwAbstractionServerAnnotation`           | [ ] Implemented | N/A       |
| `FilterDebouncingEnum`                      | [ ] Implemented | N/A       |
| `PulseTestEnum`                             | [ ] Implemented | N/A       |
| `ParameterPortAnnotation`                   | [ ] Implemented | N/A       |
| `ModePortAnnotation`                        | [ ] Implemented | N/A       |
| `TriggerPortAnnotation`                     | [ ] Implemented | N/A       |
| `NvDataPortAnnotation`                      | [ ] Implemented | N/A       |
| `DelegatedPortAnnotation`                   | [ ] Implemented | N/A       |
| `SignalFanEnum`                             | [ ] Implemented | N/A       |
| `PPortComSpec`                              | [ ] Implemented | N/A       |
| `RPortComSpec`                              | [ ] Implemented | N/A       |
| `ReceiverComSpec`                           | [ ] Implemented | N/A       |
| `HandleOutOfRangeStatusEnum`                | [ ] Implemented | N/A       |
| `NonqueuedReceiverComSpec`                  | [ ] Implemented | N/A       |
| `HandleTimeoutEnum`                         | [ ] Implemented | N/A       |
| `TimeValue`                                 | [ ] Implemented | N/A       |
| `SenderComSpec`                             | [ ] Implemented | N/A       |
| `NonqueuedSenderComSpec`                    | [ ] Implemented | N/A       |
| `TransmissionComSpecProps`                  | [ ] Implemented | N/A       |
| `TransmissionAcknowledgementRequest`        | [ ] Implemented | N/A       |
| `HandleOutOfRangeEnum`                      | [ ] Implemented | N/A       |
| `TransmissionModeDefinitionEnum`            | [ ] Implemented | N/A       |
| `ClientComSpec`                             | [ ] Implemented | N/A       |
| `ServerComSpec`                             | [ ] Implemented | N/A       |
| `ParameterProvideComSpec`                   | [ ] Implemented | N/A       |
| `TransformationComSpecProps`                | [ ] Implemented | N/A       |
| `TransformationTechnology`                  | [ ] Implemented | N/A       |

## Group28

Status: **0/75** completed

| Class Name                                   | Status          | Commit ID |
| -------------------------------------------- | --------------- | --------- |
| `BufferProperties`                           | [ ] Implemented | N/A       |
| `TransformationDescription`                  | [ ] Implemented | N/A       |
| `TransformerClassEnum`                       | [ ] Implemented | N/A       |
| `EndToEndTransformationComSpecProps`         | [ ] Implemented | N/A       |
| `E2EProfileCompatibilityProps`               | [ ] Implemented | N/A       |
| `EndToEndProtection`                         | [ ] Implemented | N/A       |
| `ConsistencyNeeds`                           | [ ] Implemented | N/A       |
| `RunnableEntityGroup`                        | [ ] Implemented | N/A       |
| `DataPrototypeGroup`                         | [ ] Implemented | N/A       |
| `SwTextProps`                                | [ ] Implemented | N/A       |
| `ApplicationArrayDataType`                   | [ ] Implemented | N/A       |
| `ApplicationArrayElement`                    | [ ] Implemented | N/A       |
| `ArraySizeSemanticsEnum`                     | [ ] Implemented | N/A       |
| `ArraySizeHandlingEnum`                      | [ ] Implemented | N/A       |
| `ImplementationDataType`                     | [ ] Implemented | N/A       |
| `SwBaseType`                                 | [ ] Implemented | N/A       |
| `BaseTypeDefinition`                         | [ ] Implemented | N/A       |
| `BaseTypeDirectDefinition`                   | [ ] Implemented | N/A       |
| `BaseType`                                   | [ ] Implemented | N/A       |
| `ByteOrderEnum`                              | [ ] Implemented | N/A       |
| `AutosarDataPrototype`                       | [ ] Implemented | N/A       |
| `ParameterInAtomicSWCTypeInstanceRef`        | [ ] Implemented | N/A       |
| `ArParameterInImplementationDataInstanceRef` | [ ] Created     | N/A       |
| `SwDataDefProps`                             | [ ] Implemented | N/A       |
| `SwBitRepresentation`                        | [ ] Implemented | N/A       |
| `SwCalibrationAccessEnum`                    | [ ] Implemented | N/A       |
| `SwCalprmAxis`                               | [ ] Implemented | N/A       |
| `CalprmAxisCategoryEnum`                     | [ ] Implemented | N/A       |
| `SwCalprmAxisTypeProps`                      | [ ] Implemented | N/A       |
| `SwAxisGeneric`                              | [ ] Implemented | N/A       |
| `SwAxisType`                                 | [ ] Created     | N/A       |
| `SwGenericAxisParam`                         | [ ] Implemented | N/A       |
| `SwCalprmRefProxy`                           | [ ] Implemented | N/A       |
| `SwVariableRefProxy`                         | [ ] Implemented | N/A       |
| `SwDataDependency`                           | [ ] Implemented | N/A       |
| `SwDataDependencyArgs`                       | [ ] Implemented | N/A       |
| `PhysicalDimension`                          | [ ] Implemented | N/A       |
| `PhysicalDimensionMapping`                   | [ ] Created     | N/A       |
| `PhysicalDimensionMappingSet`                | [ ] Created     | N/A       |
| `Unit`                                       | [ ] Implemented | N/A       |
| `PhysConstrs`                                | [ ] Implemented | N/A       |
| `InternalConstrs`                            | [ ] Implemented | N/A       |
| `Limit`                                      | [ ] Implemented | N/A       |
| `MonotonyEnum`                               | [ ] Implemented | N/A       |
| `IntervalTypeEnum`                           | [ ] Implemented | N/A       |
| `DisplayPresentationEnum`                    | [ ] Implemented | N/A       |
| `ValueSpecification`                         | [ ] Implemented | N/A       |
| `ReferenceValueSpecification`                | [ ] Implemented | N/A       |
| `NotAvailableValueSpecification`             | [ ] Implemented | N/A       |
| `ConstantSpecificationMapping`               | [ ] Implemented | N/A       |
| `ApplicationValueSpecification`              | [ ] Implemented | N/A       |
| `NumericalOrText`                            | [ ] Implemented | N/A       |
| `SwAxisCont`                                 | [ ] Created     | N/A       |
| `SwValues`                                   | [ ] Implemented | N/A       |
| `ValueGroup`                                 | [ ] Implemented | N/A       |
| `ValueList`                                  | [ ] Implemented | N/A       |
| `AbstractRuleBasedValueSpecification`        | [ ] Implemented | N/A       |
| `ApplicationRuleBasedValueSpecification`     | [ ] Implemented | N/A       |
| `RuleBasedAxisCont`                          | [ ] Implemented | N/A       |
| `RuleBasedValueCont`                         | [ ] Implemented | N/A       |
| `NumericalRuleBasedValueSpecification`       | [ ] Implemented | N/A       |
| `RuleBasedValueSpecification`                | [ ] Implemented | N/A       |
| `RuleArguments`                              | [ ] Implemented | N/A       |
| `CalibrationParameterValueSet`               | [ ] Created     | N/A       |
| `CalibrationParameterValue`                  | [ ] Created     | N/A       |
| `RunnableEntity`                             | [ ] Implemented | N/A       |
| `TimingEvent`                                | [ ] Implemented | N/A       |
| `ExecutableEntityActivationReason`           | [ ] Implemented | N/A       |
| `AbstractEvent`                              | [ ] Implemented | N/A       |
| `SwcModeSwitchEvent`                         | [ ] Implemented | N/A       |
| `ModeSwitchedAckEvent`                       | [ ] Implemented | N/A       |
| `ExternalTriggerOccurredEvent`               | [ ] Created     | N/A       |
| `TransformerHardErrorEvent`                  | [ ] Created     | N/A       |
| `OsTaskExecutionEvent`                       | [ ] Created     | N/A       |
| `WaitPoint`                                  | [ ] Implemented | N/A       |

## Group29

Status: **0/74** completed

| Class Name                                     | Status          | Commit ID |
| ---------------------------------------------- | --------------- | --------- |
| `SwcExclusiveAreaPolicy`                       | [ ] Implemented | N/A       |
| `RteApiReturnValueProvisionEnum`               | [ ] Implemented | N/A       |
| `ExternalTriggeringPoint`                      | [ ] Implemented | N/A       |
| `IncludedDataTypeSet`                          | [ ] Implemented | N/A       |
| `SwcServiceDependency`                         | [ ] Implemented | N/A       |
| `SymbolicNameProps`                            | [ ] Implemented | N/A       |
| `VariationPointProxy`                          | [ ] Implemented | N/A       |
| `SwcModeManagerErrorEvent`                     | [ ] Created     | N/A       |
| `SensorActuatorSwComponentType`                | [ ] Implemented | N/A       |
| `EcuAbstractionSwComponentType`                | [ ] Implemented | N/A       |
| `ComplexDeviceDriverSwComponentType`           | [ ] Implemented | N/A       |
| `ServiceSwComponentType`                       | [ ] Implemented | N/A       |
| `NvBlockSwComponentType`                       | [ ] Implemented | N/A       |
| `SwComponentDocumentation`                     | [ ] Implemented | N/A       |
| `AdditionalBindingTimeEnum`                    | [ ] Created     | N/A       |
| `FunctionInhibitionAvailabilityNeeds`          | [ ] Implemented | N/A       |
| `DiagnosticOperationCycleNeeds`                | [ ] Implemented | N/A       |
| `OperationCycleTypeEnum`                       | [ ] Implemented | N/A       |
| `DiagnosticEnableConditionNeeds`               | [ ] Implemented | N/A       |
| `EventAcceptanceStatusEnum`                    | [ ] Implemented | N/A       |
| `DiagnosticStorageConditionNeeds`              | [ ] Implemented | N/A       |
| `StorageConditionStatusEnum`                   | [ ] Implemented | N/A       |
| `IndicatorStatusNeeds`                         | [ ] Implemented | N/A       |
| `DiagnosticIndicatorTypeEnum`                  | [ ] Implemented | N/A       |
| `ObdRatioServiceNeeds`                         | [ ] Implemented | N/A       |
| `ObdControlServiceNeeds`                       | [ ] Implemented | N/A       |
| `ObdRatioConnectionKindEnum`                   | [ ] Implemented | N/A       |
| `ObdPidServiceNeeds`                           | [ ] Implemented | N/A       |
| `ObdInfoServiceNeeds`                          | [ ] Implemented | N/A       |
| `ObdMonitorServiceNeeds`                       | [ ] Implemented | N/A       |
| `DiagnosticMonitorUpdateKindEnum`              | [ ] Implemented | N/A       |
| `ObdRatioDenominatorNeeds`                     | [ ] Implemented | N/A       |
| `DiagnosticDenominatorConditionEnum`           | [ ] Implemented | N/A       |
| `DiagnosticTestResult`                         | [ ] Created     | N/A       |
| `DoIpRoutingActivationAuthenticationNeeds`     | [ ] Implemented | N/A       |
| `DoIpRoutingActivationConfirmationNeeds`       | [ ] Implemented | N/A       |
| `SecureOnBoardCommunicationNeeds`              | [ ] Implemented | N/A       |
| `VerificationStatusIndicationModeEnum`         | [ ] Implemented | N/A       |
| `IdsMgrNeeds`                                  | [ ] Implemented | N/A       |
| `RapidPrototypingScenario`                     | [ ] Created     | N/A       |
| `RptContainer`                                 | [ ] Created     | N/A       |
| `RptHook`                                      | [ ] Created     | N/A       |
| `RptProfile`                                   | [ ] Created     | N/A       |
| `CommunicationConnector`                       | [ ] Implemented | N/A       |
| `PhysicalChannel`                              | [ ] Implemented | N/A       |
| `AbstractCanCluster`                           | [ ] Implemented | N/A       |
| `CanCluster`                                   | [ ] Implemented | N/A       |
| `CanCommunicationController`                   | [ ] Implemented | N/A       |
| `AbstractCanCommunicationController`           | [ ] Implemented | N/A       |
| `AbstractCanCommunicationControllerAttributes` | [ ] Implemented | N/A       |
| `CanControllerFdConfiguration`                 | [ ] Implemented | N/A       |
| `CanControllerXlConfiguration`                 | [ ] Implemented | N/A       |
| `CanControllerXlConfigurationRequirements`     | [ ] Implemented | N/A       |
| `AbstractCanPhysicalChannel`                   | [ ] Implemented | N/A       |
| `CanPhysicalChannel`                           | [ ] Implemented | N/A       |
| `AbstractCanCommunicationConnector`            | [ ] Implemented | N/A       |
| `TtcanCluster`                                 | [ ] Created     | N/A       |
| `TtcanCommunicationController`                 | [ ] Created     | N/A       |
| `TtcanPhysicalChannel`                         | [ ] Created     | N/A       |
| `TtcanCommunicationConnector`                  | [ ] Created     | N/A       |
| `FlexrayCluster`                               | [ ] Implemented | N/A       |
| `FlexrayFifoConfiguration`                     | [ ] Implemented | N/A       |
| `FlexrayFifoRange`                             | [ ] Implemented | N/A       |
| `LinCluster`                                   | [ ] Implemented | N/A       |
| `LinCommunicationController`                   | [ ] Implemented | N/A       |
| `LinMaster`                                    | [ ] Implemented | N/A       |
| `LinSlaveConfig`                               | [ ] Implemented | N/A       |
| `LinSlaveConfigIdent`                          | [ ] Implemented | N/A       |
| `LinSlave`                                     | [ ] Created     | N/A       |
| `LinErrorResponse`                             | [ ] Implemented | N/A       |
| `LinConfigurableFrame`                         | [ ] Implemented | N/A       |
| `LinOrderedConfigurableFrame`                  | [ ] Implemented | N/A       |
| `LinPhysicalChannel`                           | [ ] Implemented | N/A       |
| `EthernetCluster`                              | [ ] Implemented | N/A       |

## Group30

Status: **0/75** completed

| Class Name                                       | Status          | Commit ID |
| ------------------------------------------------ | --------------- | --------- |
| `CouplingElement`                                | [ ] Created     | N/A       |
| `CouplingElementEnum`                            | [ ] Created     | N/A       |
| `CouplingPort`                                   | [ ] Implemented | N/A       |
| `EthernetConnectionNegotiationEnum`              | [ ] Implemented | N/A       |
| `EthernetMacLayerTypeEnum`                       | [ ] Implemented | N/A       |
| `EthernetPhysicalLayerTypeEnum`                  | [ ] Implemented | N/A       |
| `EthernetSwitchVlanIngressTagEnum`               | [ ] Implemented | N/A       |
| `CouplingPortConnection`                         | [ ] Implemented | N/A       |
| `EthernetCommunicationController`                | [ ] Implemented | N/A       |
| `EthernetCommunicationConnector`                 | [ ] Implemented | N/A       |
| `CouplingPortDetails`                            | [ ] Implemented | N/A       |
| `EthernetCouplingPortSchedulerEnum`              | [ ] Implemented | N/A       |
| `CouplingPortShaper`                             | [ ] Created     | N/A       |
| `CouplingPortFifo`                               | [ ] Implemented | N/A       |
| `CouplingPortRatePolicy`                         | [ ] Implemented | N/A       |
| `CouplingPortRatePolicyActionEnum`               | [ ] Implemented | N/A       |
| `CouplingPortTrafficClassAssignment`             | [ ] Implemented | N/A       |
| `EthernetSwitchVlanEgressTaggingEnum`            | [ ] Implemented | N/A       |
| `DhcpServerConfiguration`                        | [ ] Implemented | N/A       |
| `Ipv4DhcpServerConfiguration`                    | [ ] Implemented | N/A       |
| `Ipv6DhcpServerConfiguration`                    | [ ] Implemented | N/A       |
| `CouplingElementAbstractDetails`                 | [ ] Created     | N/A       |
| `CouplingElementSwitchDetails`                   | [ ] Created     | N/A       |
| `SwitchStreamIdentification`                     | [ ] Created     | N/A       |
| `SwitchStreamFilterRule`                         | [ ] Created     | N/A       |
| `StreamFilterRuleDataLinkLayer`                  | [ ] Created     | N/A       |
| `StreamFilterMACAddress`                         | [ ] Created     | N/A       |
| `StreamFilterRuleIpTp`                           | [ ] Created     | N/A       |
| `StreamFilterIpv4Address`                        | [ ] Created     | N/A       |
| `StreamFilterIpv6Address`                        | [ ] Created     | N/A       |
| `StreamFilterPortRange`                          | [ ] Created     | N/A       |
| `StreamFilterIEEE1722Tp`                         | [ ] Created     | N/A       |
| `SwitchStreamFilterActionDestPortModification`   | [ ] Created     | N/A       |
| `SwitchStreamFilterActionPortModificationEnum`   | [ ] Created     | N/A       |
| `SwitchStreamFilterEntry`                        | [ ] Created     | N/A       |
| `SwitchAsynchronousTrafficShaperGroupEntry`      | [ ] Created     | N/A       |
| `SwitchStreamGateEntry`                          | [ ] Created     | N/A       |
| `SwitchFlowMeteringEntry`                        | [ ] Created     | N/A       |
| `FlowMeteringColorModeEnum`                      | [ ] Created     | N/A       |
| `EthIpProps`                                     | [ ] Created     | N/A       |
| `Ipv4Props`                                      | [ ] Created     | N/A       |
| `Ipv4ArpProps`                                   | [ ] Created     | N/A       |
| `Ipv4AutoIpProps`                                | [ ] Created     | N/A       |
| `Ipv4FragmentationProps`                         | [ ] Created     | N/A       |
| `Ipv6Props`                                      | [ ] Created     | N/A       |
| `Ipv6FragmentationProps`                         | [ ] Created     | N/A       |
| `Dhcpv6Props`                                    | [ ] Created     | N/A       |
| `Ipv6NdpProps`                                   | [ ] Created     | N/A       |
| `EthernetWakeupSleepOnDatalineConfig`            | [ ] Created     | N/A       |
| `EthernetWakeupSleepOnDatalineConfigSet`         | [ ] Created     | N/A       |
| `PlcaProps`                                      | [ ] Implemented | N/A       |
| `MacSecProps`                                    | [ ] Implemented | N/A       |
| `MacSecLocalKayProps`                            | [ ] Implemented | N/A       |
| `MacSecGlobalKayProps`                           | [ ] Implemented | N/A       |
| `MacSecParticipantSet`                           | [ ] Created     | N/A       |
| `MacSecKayParticipant`                           | [ ] Implemented | N/A       |
| `MacSecCryptoAlgoConfig`                         | [ ] Implemented | N/A       |
| `MacSecCipherSuiteConfig`                        | [ ] Implemented | N/A       |
| `MacSecConfidentialityOffsetEnum`                | [ ] Implemented | N/A       |
| `MacSecCapabilityEnum`                           | [ ] Implemented | N/A       |
| `MacSecRoleEnum`                                 | [ ] Implemented | N/A       |
| `MacSecFailPermissiveModeEnum`                   | [ ] Implemented | N/A       |
| `UserDefinedCluster`                             | [ ] Created     | N/A       |
| `UserDefinedPhysicalChannel`                     | [ ] Created     | N/A       |
| `UserDefinedCommunicationConnector`              | [ ] Created     | N/A       |
| `UserDefinedCommunicationController`             | [ ] Created     | N/A       |
| `SystemMapping`                                  | [ ] Implemented | N/A       |
| `SwcToApplicationPartitionMapping`               | [ ] Created     | N/A       |
| `ApplicationPartition`                           | [ ] Created     | N/A       |
| `MappingConstraint`                              | [ ] Created     | N/A       |
| `ComponentClustering`                            | [ ] Created     | N/A       |
| `MappingScopeEnum`                               | [ ] Created     | N/A       |
| `ComponentSeparation`                            | [ ] Created     | N/A       |
| `J1939ControllerApplicationToJ1939NmNodeMapping` | [ ] Created     | N/A       |
| `J1939ControllerApplication`                     | [ ] Created     | N/A       |

## Group31

Status: **0/75** completed

| Class Name                                               | Status          | Commit ID |
| -------------------------------------------------------- | --------------- | --------- |
| `RteEventInCompositionToOsTaskProxyMapping`              | [ ] Created     | N/A       |
| `RteEventInCompositionSeparation`                        | [ ] Created     | N/A       |
| `RteEventInSystemToOsTaskProxyMapping`                   | [ ] Created     | N/A       |
| `RteEventInSystemSeparation`                             | [ ] Created     | N/A       |
| `SenderRecArrayElementMapping`                           | [ ] Implemented | N/A       |
| `ClientServerToSignalMapping`                            | [ ] Created     | N/A       |
| `SenderReceiverCompositeElementToSignalMapping`          | [ ] Created     | N/A       |
| `TriggerToSignalMapping`                                 | [ ] Created     | N/A       |
| `CommonSignalPath`                                       | [ ] Created     | N/A       |
| `SwcToSwcSignal`                                         | [ ] Created     | N/A       |
| `SwcToSwcOperationArguments`                             | [ ] Created     | N/A       |
| `SwcToSwcOperationArgumentsDirectionEnum`                | [ ] Created     | N/A       |
| `ForbiddenSignalPath`                                    | [ ] Created     | N/A       |
| `PermissibleSignalPath`                                  | [ ] Created     | N/A       |
| `SeparateSignalPath`                                     | [ ] Created     | N/A       |
| `EcuResourceEstimation`                                  | [ ] Created     | N/A       |
| `PncMapping`                                             | [ ] Created     | N/A       |
| `CpSoftwareClusterToEcuInstanceMapping`                  | [ ] Created     | N/A       |
| `CpSoftwareClusterResourceToApplicationPartitionMapping` | [ ] Created     | N/A       |
| `CpSoftwareClusterMappingSet`                            | [ ] Created     | N/A       |
| `CpSoftwareClusterToApplicationPartitionMapping`         | [ ] Created     | N/A       |
| `SystemSignalToCommunicationResourceMapping`             | [ ] Created     | N/A       |
| `SystemSignalGroupToCommunicationResourceMapping`        | [ ] Created     | N/A       |
| `DdsCpISignalToDdsTopicMapping`                          | [ ] Created     | N/A       |
| `CommConnectorPort`                                      | [ ] Implemented | N/A       |
| `IPduPort`                                               | [ ] Implemented | N/A       |
| `IPduSignalProcessingEnum`                               | [ ] Implemented | N/A       |
| `ISignal`                                                | [ ] Implemented | N/A       |
| `DataTypePolicyEnum`                                     | [ ] Implemented | N/A       |
| `ISignalTypeEnum`                                        | [ ] Implemented | N/A       |
| `ISignalProps`                                           | [ ] Implemented | N/A       |
| `ISignalGroup`                                           | [ ] Implemented | N/A       |
| `SystemSignalGroup`                                      | [ ] Implemented | N/A       |
| `ISignalToIPduMapping`                                   | [ ] Implemented | N/A       |
| `ISignalTriggering`                                      | [ ] Implemented | N/A       |
| `Pdu`                                                    | [ ] Implemented | N/A       |
| `IPdu`                                                   | [ ] Implemented | N/A       |
| `ISignalIPdu`                                            | [ ] Implemented | N/A       |
| `NmPdu`                                                  | [ ] Implemented | N/A       |
| `NPdu`                                                   | [ ] Implemented | N/A       |
| `DcmIPdu`                                                | [ ] Implemented | N/A       |
| `DiagPduType`                                            | [ ] Created     | N/A       |
| `J1939DcmIPdu`                                           | [ ] Created     | N/A       |
| `PduToFrameMapping`                                      | [ ] Implemented | N/A       |
| `IPduTiming`                                             | [ ] Implemented | N/A       |
| `PduTriggering`                                          | [ ] Implemented | N/A       |
| `ContainerIPdu`                                          | [ ] Created     | N/A       |
| `ContainerIPduTriggerEnum`                               | [ ] Created     | N/A       |
| `ContainerIPduHeaderTypeEnum`                            | [ ] Created     | N/A       |
| `RxAcceptContainedIPduEnum`                              | [ ] Created     | N/A       |
| `SecureCommunicationProps`                               | [ ] Implemented | N/A       |
| `SecureCommunicationPropsSet`                            | [ ] Implemented | N/A       |
| `SecureCommunicationFreshnessProps`                      | [ ] Implemented | N/A       |
| `SecureCommunicationAuthenticationProps`                 | [ ] Implemented | N/A       |
| `CryptoServiceKey`                                       | [ ] Created     | N/A       |
| `CryptoServiceKeyGenerationEnum`                         | [ ] Created     | N/A       |
| `CryptoServiceQueue`                                     | [ ] Created     | N/A       |
| `GeneralPurposeConnection`                               | [ ] Created     | N/A       |
| `RelativeTolerance`                                      | [ ] Created     | N/A       |
| `AbsoluteTolerance`                                      | [ ] Created     | N/A       |
| `Frame`                                                  | [ ] Implemented | N/A       |
| `LinFrame`                                               | [ ] Implemented | N/A       |
| `LinFrameTriggering`                                     | [ ] Implemented | N/A       |
| `LinChecksumType`                                        | [ ] Created     | N/A       |
| `LinUnconditionalFrame`                                  | [ ] Implemented | N/A       |
| `LinSporadicFrame`                                       | [ ] Created     | N/A       |
| `LinEventTriggeredFrame`                                 | [ ] Created     | N/A       |
| `ScheduleTableEntry`                                     | [ ] Implemented | N/A       |
| `FreeFormatEntry`                                        | [ ] Implemented | N/A       |
| `LinConfigurationEntry`                                  | [ ] Implemented | N/A       |
| `AssignFrameId`                                          | [ ] Implemented | N/A       |
| `UnassignFrameId`                                        | [ ] Implemented | N/A       |
| `AssignFrameIdRange`                                     | [ ] Implemented | N/A       |
| `FramePid`                                               | [ ] Implemented | N/A       |
| `AssignNad`                                              | [ ] Implemented | N/A       |

## Group32

Status: **0/75** completed

| Class Name                             | Status          | Commit ID |
| -------------------------------------- | --------------- | --------- |
| `ConditionalChangeNad`                 | [ ] Implemented | N/A       |
| `SaveConfigurationEntry`               | [ ] Implemented | N/A       |
| `DataDumpEntry`                        | [ ] Implemented | N/A       |
| `FreeFormat`                           | [ ] Implemented | N/A       |
| `CanFrame`                             | [ ] Implemented | N/A       |
| `CanFrameTriggering`                   | [ ] Implemented | N/A       |
| `CanAddressingModeType`                | [ ] Implemented | N/A       |
| `RxIdentifierRange`                    | [ ] Implemented | N/A       |
| `CanFrameRxBehaviorEnum`               | [ ] Implemented | N/A       |
| `CanFrameTxBehaviorEnum`               | [ ] Implemented | N/A       |
| `TtcanAbsolutelyScheduledTiming`       | [ ] Implemented | N/A       |
| `TtcanTriggerType`                     | [ ] Implemented | N/A       |
| `SoAdConfig`                           | [ ] Implemented | N/A       |
| `SocketAddress`                        | [ ] Implemented | N/A       |
| `UdpChecksumCalculationEnum`           | [ ] Implemented | N/A       |
| `IPv6ExtHeaderFilterSet`               | [ ] Created     | N/A       |
| `ApplicationEndpoint`                  | [ ] Implemented | N/A       |
| `RtpTp`                                | [ ] Created     | N/A       |
| `Ieee1722Tp`                           | [ ] Created     | N/A       |
| `HttpTp`                               | [ ] Created     | N/A       |
| `Ipv6Configuration`                    | [ ] Implemented | N/A       |
| `MacMulticastConfiguration`            | [ ] Created     | N/A       |
| `InfrastructureServices`               | [ ] Implemented | N/A       |
| `TimeSyncTechnologyEnum`               | [ ] Implemented | N/A       |
| `DoIpEntityRoleEnum`                   | [ ] Implemented | N/A       |
| `DdsCpServiceInstance`                 | [ ] Created     | N/A       |
| `DdsCpProvidedServiceInstance`         | [ ] Created     | N/A       |
| `DdsCpConsumedServiceInstance`         | [ ] Created     | N/A       |
| `DdsCpServiceInstanceEvent`            | [ ] Created     | N/A       |
| `DdsCpServiceInstanceOperation`        | [ ] Created     | N/A       |
| `ServiceInstanceCollectionSet`         | [ ] Created     | N/A       |
| `AbstractServiceInstance`              | [ ] Implemented | N/A       |
| `ProvidedServiceInstance`              | [ ] Implemented | N/A       |
| `PduActivationRoutingGroup`            | [ ] Implemented | N/A       |
| `EventGroupControlTypeEnum`            | [ ] Implemented | N/A       |
| `SoConIPduIdentifier`                  | [ ] Created     | N/A       |
| `SocketConnectionIpduIdentifierSet`    | [ ] Created     | N/A       |
| `EventHandler`                         | [ ] Implemented | N/A       |
| `ConsumedServiceInstance`              | [ ] Implemented | N/A       |
| `ConsumedEventGroup`                   | [ ] Implemented | N/A       |
| `SomeipSdServerServiceInstanceConfig`  | [ ] Created     | N/A       |
| `SomeipSdServerEventGroupTimingConfig` | [ ] Implemented | N/A       |
| `SomeipSdClientEventGroupTimingConfig` | [ ] Implemented | N/A       |
| `DdsCpConfig`                          | [ ] Created     | N/A       |
| `DdsCpDomain`                          | [ ] Created     | N/A       |
| `DdsCpTopic`                           | [ ] Created     | N/A       |
| `DdsCpPartition`                       | [ ] Created     | N/A       |
| `DdsCpQosProfile`                      | [ ] Created     | N/A       |
| `DdsTopicData`                         | [ ] Created     | N/A       |
| `DdsDurability`                        | [ ] Created     | N/A       |
| `DdsDurabilityKindEnum`                | [ ] Created     | N/A       |
| `DdsDurabilityService`                 | [ ] Created     | N/A       |
| `DdsDurabilityServiceHistoryKindEnum`  | [ ] Created     | N/A       |
| `DdsDeadline`                          | [ ] Created     | N/A       |
| `DdsLatencyBudget`                     | [ ] Created     | N/A       |
| `DdsOwnership`                         | [ ] Created     | N/A       |
| `DdsOwnershipKindEnum`                 | [ ] Created     | N/A       |
| `DdsOwnershipStrength`                 | [ ] Created     | N/A       |
| `DdsLiveliness`                        | [ ] Created     | N/A       |
| `DdsLivenessKindEnum`                  | [ ] Created     | N/A       |
| `DdsReliability`                       | [ ] Created     | N/A       |
| `DdsReliabilityKindEnum`               | [ ] Created     | N/A       |
| `DdsTransportPriority`                 | [ ] Created     | N/A       |
| `DdsLifespan`                          | [ ] Created     | N/A       |
| `DdsDestinationOrder`                  | [ ] Created     | N/A       |
| `DdsDestinationOrderKindEnum`          | [ ] Created     | N/A       |
| `DdsHistory`                           | [ ] Created     | N/A       |
| `DdsHistoryKindEnum`                   | [ ] Created     | N/A       |
| `DdsResourceLimits`                    | [ ] Created     | N/A       |
| `StaticSocketConnection`               | [ ] Implemented | N/A       |
| `IPSecRule`                            | [ ] Implemented | N/A       |
| `IPSecConfigProps`                     | [ ] Implemented | N/A       |
| `IPsecIpProtocolEnum`                  | [ ] Implemented | N/A       |
| `IPsecPolicyEnum`                      | [ ] Implemented | N/A       |
| `IPsecModeEnum`                        | [ ] Implemented | N/A       |

## Group33

Status: **0/75** completed

| Class Name                                  | Status          | Commit ID |
| ------------------------------------------- | --------------- | --------- |
| `IPsecHeaderTypeEnum`                       | [ ] Implemented | N/A       |
| `IPsecDpdActionEnum`                        | [ ] Implemented | N/A       |
| `EthernetFrameTriggering`                   | [ ] Created     | N/A       |
| `UserDefinedEthernetFrame`                  | [ ] Created     | N/A       |
| `Ieee1722TpEthernetFrame`                   | [ ] Created     | N/A       |
| `StateDependentFirewall`                    | [ ] Implemented | N/A       |
| `TpConfig`                                  | [ ] Implemented | N/A       |
| `FlexrayTpConfig`                           | [ ] Created     | N/A       |
| `FlexrayTpConnectionControl`                | [ ] Created     | N/A       |
| `FlexrayTpConnection`                       | [ ] Created     | N/A       |
| `FlexrayTpPduPool`                          | [ ] Created     | N/A       |
| `FlexrayTpNode`                             | [ ] Created     | N/A       |
| `FlexrayTpEcu`                              | [ ] Created     | N/A       |
| `FlexrayArTpConfig`                         | [ ] Created     | N/A       |
| `FlexrayArTpChannel`                        | [ ] Created     | N/A       |
| `FlexrayArTpNode`                           | [ ] Created     | N/A       |
| `FlexrayArTpConnection`                     | [ ] Created     | N/A       |
| `FrArTpAckType`                             | [ ] Created     | N/A       |
| `MaximumMessageLengthType`                  | [ ] Created     | N/A       |
| `CanTpConfig`                               | [ ] Implemented | N/A       |
| `CanTpChannel`                              | [ ] Implemented | N/A       |
| `CanTpConnection`                           | [ ] Implemented | N/A       |
| `CanTpAddressingFormatType`                 | [ ] Implemented | N/A       |
| `CanTpAddress`                              | [ ] Implemented | N/A       |
| `CanTpEcu`                                  | [ ] Implemented | N/A       |
| `CanTpNode`                                 | [ ] Implemented | N/A       |
| `NetworkTargetAddressType`                  | [ ] Implemented | N/A       |
| `LinTpConfig`                               | [ ] Implemented | N/A       |
| `LinTpNode`                                 | [ ] Implemented | N/A       |
| `EthTpConfig`                               | [ ] Created     | N/A       |
| `EthTpConnection`                           | [ ] Created     | N/A       |
| `SomeipTpConfig`                            | [ ] Created     | N/A       |
| `SomeipTpConnection`                        | [ ] Created     | N/A       |
| `SomeipTpChannel`                           | [ ] Created     | N/A       |
| `J1939TpConfig`                             | [ ] Created     | N/A       |
| `J1939TpConnection`                         | [ ] Created     | N/A       |
| `J1939TpPg`                                 | [ ] Created     | N/A       |
| `J1939TpNode`                               | [ ] Created     | N/A       |
| `TpConnection`                              | [ ] Implemented | N/A       |
| `IEEE1722TpConfig`                          | [ ] Created     | N/A       |
| `IEEE1722TpConnection`                      | [ ] Created     | N/A       |
| `IEEE1722TpAvConnection`                    | [ ] Created     | N/A       |
| `IEEE1722TpCrfConnection`                   | [ ] Created     | N/A       |
| `IEEE1722TpCrfTypeEnum`                     | [ ] Created     | N/A       |
| `IEEE1722TpCrfPullEnum`                     | [ ] Created     | N/A       |
| `IEEE1722TpAafConnection`                   | [ ] Created     | N/A       |
| `IEEE1722TpAafNominalRateEnum`              | [ ] Created     | N/A       |
| `IEEE1722TpAafFormatEnum`                   | [ ] Created     | N/A       |
| `IEEE1722TpAafAes3DataTypeEnum`             | [ ] Created     | N/A       |
| `IEEE1722TpIidcConnection`                  | [ ] Created     | N/A       |
| `IEEE1722TpRvfConnection`                   | [ ] Created     | N/A       |
| `IEEE1722TpRvfPixelDepthEnum`               | [ ] Created     | N/A       |
| `IEEE1722TpRvfPixelFormatEnum`              | [ ] Created     | N/A       |
| `IEEE1722TpRvfColorSpaceEnum`               | [ ] Created     | N/A       |
| `IEEE1722TpRvfFrameRateEnum`                | [ ] Created     | N/A       |
| `IEEE1722TpAcfConnection`                   | [ ] Created     | N/A       |
| `IEEE1722TpAcfBus`                          | [ ] Created     | N/A       |
| `IEEE1722TpAcfBusPart`                      | [ ] Created     | N/A       |
| `IEEE1722TpAcfCan`                          | [ ] Created     | N/A       |
| `IEEE1722TpAcfCanPart`                      | [ ] Created     | N/A       |
| `IEEE1722TpAcfCanMessageTypeEnum`           | [ ] Created     | N/A       |
| `IEEE1722TpAcfLin`                          | [ ] Created     | N/A       |
| `IEEE1722TpAcfLinPart`                      | [ ] Created     | N/A       |
| `BusspecificNmEcu`                          | [ ] Implemented | N/A       |
| `NmCoordinator`                             | [ ] Created     | N/A       |
| `NmNode`                                    | [ ] Implemented | N/A       |
| `NmCoordinatorRoleEnum`                     | [ ] Implemented | N/A       |
| `FlexrayNmScheduleVariant`                  | [ ] Implemented | N/A       |
| `CanNmEcu`                                  | [ ] Implemented | N/A       |
| `J1939NmNode`                               | [ ] Implemented | N/A       |
| `J1939NodeName`                             | [ ] Implemented | N/A       |
| `J1939NmAddressConfigurationCapabilityEnum` | [ ] Implemented | N/A       |
| `BusMirrorChannelMapping`                   | [ ] Created     | N/A       |
| `MirroringProtocolEnum`                     | [ ] Created     | N/A       |
| `BusMirrorChannel`                          | [ ] Created     | N/A       |

## Group34

Status: **0/75** completed

| Class Name                                          | Status          | Commit ID |
| --------------------------------------------------- | --------------- | --------- |
| `BusMirrorChannelMappingCan`                        | [ ] Created     | N/A       |
| `BusMirrorCanIdRangeMapping`                        | [ ] Created     | N/A       |
| `BusMirrorCanIdToCanIdMapping`                      | [ ] Created     | N/A       |
| `BusMirrorLinPidToCanIdMapping`                     | [ ] Created     | N/A       |
| `BusMirrorChannelMappingFlexray`                    | [ ] Created     | N/A       |
| `BusMirrorChannelMappingIp`                         | [ ] Created     | N/A       |
| `BusMirrorChannelMappingUserDefined`                | [ ] Created     | N/A       |
| `SignalServiceTranslationPropsSet`                  | [ ] Implemented | N/A       |
| `SignalServiceTranslationProps`                     | [ ] Implemented | N/A       |
| `SignalServiceTranslationEventProps`                | [ ] Implemented | N/A       |
| `SignalServiceTranslationControlEnum`               | [ ] Implemented | N/A       |
| `UserDefinedTransformationDescription`              | [ ] Created     | N/A       |
| `CSTransformerErrorReactionEnum`                    | [ ] Implemented | N/A       |
| `SOMEIPTransformationDescription`                   | [ ] Created     | N/A       |
| `TransformationPropsSet`                            | [ ] Created     | N/A       |
| `TransformationProps`                               | [ ] Created     | N/A       |
| `SOMEIPTransformationProps`                         | [ ] Created     | N/A       |
| `DataPrototypeReference`                            | [ ] Implemented | N/A       |
| `DataPrototypeInPortInterfaceRef`                   | [ ] Implemented | N/A       |
| `DataPrototypeInSenderReceiverInterfaceInstanceRef` | [ ] Implemented | N/A       |
| `DataPrototypeInClientServerInterfaceInstanceRef`   | [ ] Implemented | N/A       |
| `ImplementationDataTypeElementInPortInterfaceRef`   | [ ] Implemented | N/A       |
| `EndToEndTransformationDescription`                 | [ ] Implemented | N/A       |
| `DataIdModeEnum`                                    | [ ] Implemented | N/A       |
| `EndToEndProfileBehaviorEnum`                       | [ ] Implemented | N/A       |
| `UserDefinedTransformationProps`                    | [ ] Created     | N/A       |
| `GlobalTimeDomain`                                  | [ ] Created     | N/A       |
| `AbstractGlobalTimeDomainProps`                     | [ ] Created     | N/A       |
| `NetworkSegmentIdentification`                      | [ ] Created     | N/A       |
| `GlobalTimeMaster`                                  | [ ] Created     | N/A       |
| `GlobalTimeSlave`                                   | [ ] Created     | N/A       |
| `GlobalTimeGateway`                                 | [ ] Created     | N/A       |
| `GlobalTimeCorrectionProps`                         | [ ] Created     | N/A       |
| `GlobalTimeCanMaster`                               | [ ] Created     | N/A       |
| `GlobalTimeCanSlave`                                | [ ] Created     | N/A       |
| `CanGlobalTimeDomainProps`                          | [ ] Created     | N/A       |
| `GlobalTimeEthMaster`                               | [ ] Created     | N/A       |
| `EthTSynSubTlvConfig`                               | [ ] Created     | N/A       |
| `GlobalTimeEthSlave`                                | [ ] Created     | N/A       |
| `EthGlobalTimeDomainProps`                          | [ ] Created     | N/A       |
| `EthTSynCrcFlags`                                   | [ ] Created     | N/A       |
| `EthGlobalTimeMessageFormatEnum`                    | [ ] Created     | N/A       |
| `EthGlobalTimeManagedCouplingPort`                  | [ ] Created     | N/A       |
| `GlobalTimeCouplingPortProps`                       | [ ] Implemented | N/A       |
| `GlobalTimePortRoleEnum`                            | [ ] Created     | N/A       |
| `GlobalTimeFrMaster`                                | [ ] Created     | N/A       |
| `GlobalTimeFrSlave`                                 | [ ] Created     | N/A       |
| `FrGlobalTimeDomainProps`                           | [ ] Created     | N/A       |
| `UserDefinedGlobalTimeMaster`                       | [ ] Created     | N/A       |
| `UserDefinedGlobalTimeSlave`                        | [ ] Created     | N/A       |
| `GlobalTimeCrcSupportEnum`                          | [ ] Created     | N/A       |
| `GlobalTimeCrcValidationEnum`                       | [ ] Created     | N/A       |
| `GlobalTimeIcvSupportEnum`                          | [ ] Created     | N/A       |
| `GlobalTimeIcvVerificationEnum`                     | [ ] Created     | N/A       |
| `CpSoftwareClusterResourcePool`                     | [ ] Created     | N/A       |
| `CpSoftwareClusterCommunicationResource`            | [ ] Created     | N/A       |
| `CpSoftwareClusterCommunicationResourceProps`       | [ ] Created     | N/A       |
| `DataComProps`                                      | [ ] Created     | N/A       |
| `DataConsistencyPolicyEnum`                         | [ ] Created     | N/A       |
| `ClientServerOperationComProps`                     | [ ] Created     | N/A       |
| `SendIndicationEnum`                                | [ ] Created     | N/A       |
| `CpSoftwareClusterServiceResource`                  | [ ] Created     | N/A       |
| `PortElementToCommunicationResourceMapping`         | [ ] Created     | N/A       |
| `CpSoftwareClusterToResourceMapping`                | [ ] Created     | N/A       |
| `CpSoftwareClusterBinaryManifestDescriptor`         | [ ] Created     | N/A       |
| `BinaryManifestProvideResource`                     | [ ] Created     | N/A       |
| `BinaryManifestResource`                            | [ ] Created     | N/A       |
| `BinaryManifestRequireResource`                     | [ ] Created     | N/A       |
| `BinaryManifestResourceDefinition`                  | [ ] Created     | N/A       |
| `BinaryManifestItem`                                | [ ] Created     | N/A       |
| `BinaryManifestItemDefinition`                      | [ ] Created     | N/A       |
| `BinaryManifestAddressableObject`                   | [ ] Created     | N/A       |
| `BinaryManifestItemValue`                           | [ ] Created     | N/A       |
| `BinaryManifestItemNumericalValue`                  | [ ] Created     | N/A       |
| `BinaryManifestItemPointerValue`                    | [ ] Created     | N/A       |

## Group35

Status: **0/75** completed

| Class Name                             | Status          | Commit ID |
| -------------------------------------- | --------------- | --------- |
| `BinaryManifestMetaDataField`          | [ ] Created     | N/A       |
| `VfbTiming`                            | [ ] Created     | N/A       |
| `SwcTiming`                            | [ ] Implemented | N/A       |
| `SystemTiming`                         | [ ] Created     | N/A       |
| `BswModuleTiming`                      | [ ] Created     | N/A       |
| `BswCompositionTiming`                 | [ ] Created     | N/A       |
| `EcuTiming`                            | [ ] Created     | N/A       |
| `TimingCondition`                      | [ ] Implemented | N/A       |
| `TimingConditionFormula`               | [ ] Implemented | N/A       |
| `TimingExtensionResource`              | [ ] Implemented | N/A       |
| `TimingModeInstance`                   | [ ] Implemented | N/A       |
| `ModeInBswInstanceRef`                 | [ ] Implemented | N/A       |
| `TDEventVfbReference`                  | [ ] Implemented | N/A       |
| `TDEventVfbPort`                       | [ ] Implemented | N/A       |
| `TDEventVariableDataPrototype`         | [ ] Implemented | N/A       |
| `TDEventVariableDataPrototypeTypeEnum` | [ ] Implemented | N/A       |
| `TDEventOperation`                     | [ ] Implemented | N/A       |
| `TDEventOperationTypeEnum`             | [ ] Implemented | N/A       |
| `TDEventModeDeclaration`               | [ ] Implemented | N/A       |
| `TDEventModeDeclarationTypeEnum`       | [ ] Implemented | N/A       |
| `TDEventTrigger`                       | [ ] Implemented | N/A       |
| `TDEventTriggerTypeEnum`               | [ ] Implemented | N/A       |
| `TDEventSwc`                           | [ ] Implemented | N/A       |
| `TDEventSwcInternalBehavior`           | [ ] Implemented | N/A       |
| `TDEventSwcInternalBehaviorTypeEnum`   | [ ] Implemented | N/A       |
| `TDEventSwcInternalBehaviorReference`  | [ ] Implemented | N/A       |
| `TDEventCom`                           | [ ] Implemented | N/A       |
| `TDEventISignal`                       | [ ] Implemented | N/A       |
| `TDEventISignalTypeEnum`               | [ ] Implemented | N/A       |
| `TDEventIPdu`                          | [ ] Implemented | N/A       |
| `TDEventIPduTypeEnum`                  | [ ] Implemented | N/A       |
| `TDEventFrame`                         | [ ] Implemented | N/A       |
| `TDEventFrameTypeEnum`                 | [ ] Implemented | N/A       |
| `TDEventFrameEthernet`                 | [ ] Implemented | N/A       |
| `TDEventFrameEthernetTypeEnum`         | [ ] Implemented | N/A       |
| `TDHeaderIdRange`                      | [ ] Implemented | N/A       |
| `TDEventCycleStart`                    | [ ] Implemented | N/A       |
| `TDEventFrClusterCycleStart`           | [ ] Implemented | N/A       |
| `TDEventTTCanCycleStart`               | [ ] Implemented | N/A       |
| `TDEventBswInternalBehavior`           | [ ] Implemented | N/A       |
| `TDEventBswInternalBehaviorTypeEnum`   | [ ] Implemented | N/A       |
| `TDEventBswModule`                     | [ ] Implemented | N/A       |
| `TDEventBswModuleTypeEnum`             | [ ] Implemented | N/A       |
| `TDEventBswModeDeclaration`            | [ ] Implemented | N/A       |
| `TDEventBswModeDeclarationTypeEnum`    | [ ] Implemented | N/A       |
| `TDEventComplex`                       | [ ] Implemented | N/A       |
| `TDEventSLLETPort`                     | [ ] Implemented | N/A       |
| `TDEventOccurrenceExpression`          | [ ] Implemented | N/A       |
| `TDEventOccurrenceExpressionFormula`   | [ ] Implemented | N/A       |
| `AutosarVariableInstance`              | [ ] Implemented | N/A       |
| `SynchronizationTypeEnum`              | [ ] Implemented | N/A       |
| `EventOccurrenceKindEnum`              | [ ] Implemented | N/A       |
| `LatencyTimingConstraint`              | [ ] Implemented | N/A       |
| `LatencyConstraintTypeEnum`            | [ ] Implemented | N/A       |
| `EventTriggeringConstraint`            | [ ] Implemented | N/A       |
| `PeriodicEventTriggering`              | [ ] Implemented | N/A       |
| `SporadicEventTriggering`              | [ ] Implemented | N/A       |
| `ConcretePatternEventTriggering`       | [ ] Implemented | N/A       |
| `BurstPatternEventTriggering`          | [ ] Implemented | N/A       |
| `ArbitraryEventTriggering`             | [ ] Implemented | N/A       |
| `ConfidenceInterval`                   | [ ] Implemented | N/A       |
| `AgeConstraint`                        | [ ] Implemented | N/A       |
| `ExecutionOrderConstraint`             | [ ] Implemented | N/A       |
| `ExecutionOrderConstraintTypeEnum`     | [ ] Implemented | N/A       |
| `EOCExecutableEntityRefAbstract`       | [ ] Implemented | N/A       |
| `EOCExecutableEntityRefGroup`          | [ ] Implemented | N/A       |
| `EOCExecutableEntityRef`               | [ ] Implemented | N/A       |
| `EOCEventRef`                          | [ ] Implemented | N/A       |
| `ExecutionTimeConstraint`              | [ ] Implemented | N/A       |
| `ExecutionTimeTypeEnum`                | [ ] Implemented | N/A       |
| `SynchronizationPointConstraint`       | [ ] Implemented | N/A       |
| `LetDataExchangeParadigmEnum`          | [ ] Implemented | N/A       |
| `TDCpSoftwareClusterMappingSet`        | [ ] Created     | N/A       |
| `TDCpSoftwareClusterMapping`           | [ ] Created     | N/A       |
| `TDCpSoftwareClusterResourceMapping`   | [ ] Created     | N/A       |

## Group36

Status: **0/73** completed

| Class Name                                     | Status          | Commit ID |
| ---------------------------------------------- | --------------- | --------- |
| `ApplicationInterface`                         | [ ] Implemented | N/A       |
| `FMFeatureModel`                               | [ ] Created     | N/A       |
| `FMFeature`                                    | [ ] Created     | N/A       |
| `FMAttributeDef`                               | [ ] Created     | N/A       |
| `FMFeatureDecomposition`                       | [ ] Created     | N/A       |
| `FMFeatureRestriction`                         | [ ] Created     | N/A       |
| `FMFeatureRelation`                            | [ ] Created     | N/A       |
| `FMFeatureSelection`                           | [ ] Created     | N/A       |
| `FMFeatureSelectionState`                      | [ ] Created     | N/A       |
| `FMAttributeValue`                             | [ ] Created     | N/A       |
| `FMFeatureSelectionSet`                        | [ ] Created     | N/A       |
| `FMFeatureMap`                                 | [ ] Created     | N/A       |
| `FMFeatureMapElement`                          | [ ] Created     | N/A       |
| `FMFeatureMapCondition`                        | [ ] Created     | N/A       |
| `FMFeatureMapAssertion`                        | [ ] Created     | N/A       |
| `SwSystemconstantValueSet`                     | [ ] Implemented | N/A       |
| `PostBuildVariantCriterionValueSet`            | [ ] Created     | N/A       |
| `LogAndTraceMessageCollectionSet`              | [ ] Created     | N/A       |
| `IdsDesign`                                    | [ ] Created     | N/A       |
| `SecurityEventDefinition`                      | [ ] Created     | N/A       |
| `SecurityEventFilterChain`                     | [ ] Created     | N/A       |
| `AbstractSecurityEventFilter`                  | [ ] Created     | N/A       |
| `SecurityEventStateFilter`                     | [ ] Created     | N/A       |
| `SecurityEventOneEveryNFilter`                 | [ ] Created     | N/A       |
| `SecurityEventAggregationFilter`               | [ ] Created     | N/A       |
| `SecurityEventContextDataSourceEnum`           | [ ] Created     | N/A       |
| `SecurityEventThresholdFilter`                 | [ ] Created     | N/A       |
| `IdsmRateLimitation`                           | [ ] Created     | N/A       |
| `IdsmTrafficLimitation`                        | [ ] Created     | N/A       |
| `SecurityEventContextMapping`                  | [ ] Created     | N/A       |
| `SecurityEventContextProps`                    | [ ] Created     | N/A       |
| `SecurityEventReportingModeEnum`               | [ ] Created     | N/A       |
| `SecurityEventContextMappingBswModule`         | [ ] Created     | N/A       |
| `SecurityEventContextMappingFunctionalCluster` | [ ] Created     | N/A       |
| `SecurityEventContextMappingCommConnector`     | [ ] Created     | N/A       |
| `SecurityEventContextMappingApplication`       | [ ] Created     | N/A       |
| `IdsmInstance`                                 | [ ] Created     | N/A       |
| `BlockState`                                   | [ ] Created     | N/A       |
| `ClientServerOperationBlueprintMapping`        | [ ] Created     | N/A       |
| `DataExchangePoint`                            | [ ] Created     | N/A       |
| `Baseline`                                     | [ ] Created     | N/A       |
| `DataExchangePointKind`                        | [ ] Created     | N/A       |
| `SpecElementReference`                         | [ ] Created     | N/A       |
| `SpecElementScope`                             | [ ] Created     | N/A       |
| `RestrictionWithSeverity`                      | [ ] Created     | N/A       |
| `SeverityEnum`                                 | [ ] Created     | N/A       |
| `ValueRestrictionWithSeverity`                 | [ ] Created     | N/A       |
| `MultiplicityRestrictionWithSeverity`          | [ ] Created     | N/A       |
| `AbstractMultiplicityRestriction`              | [ ] Created     | N/A       |
| `VariationRestrictionWithSeverity`             | [ ] Created     | N/A       |
| `DataFormatElementReference`                   | [ ] Created     | N/A       |
| `DataFormatElementScope`                       | [ ] Created     | N/A       |
| `SpecificationScope`                           | [ ] Created     | N/A       |
| `SpecificationDocumentScope`                   | [ ] Created     | N/A       |
| `DocumentElementScope`                         | [ ] Created     | N/A       |
| `AbstractClassTailoring`                       | [ ] Created     | N/A       |
| `AbstractCondition`                            | [ ] Created     | N/A       |
| `AggregationCondition`                         | [ ] Created     | N/A       |
| `AttributeCondition`                           | [ ] Created     | N/A       |
| `ClassTailoring`                               | [ ] Created     | N/A       |
| `ClassContentConditional`                      | [ ] Created     | N/A       |
| `ConcreteClassTailoring`                       | [ ] Created     | N/A       |
| `InvertCondition`                              | [ ] Created     | N/A       |
| `PrimitiveAttributeCondition`                  | [ ] Created     | N/A       |
| `ReferenceCondition`                           | [ ] Created     | N/A       |
| `TextualCondition`                             | [ ] Created     | N/A       |
| `AttributeTailoring`                           | [ ] Created     | N/A       |
| `PrimitiveAttributeTailoring`                  | [ ] Created     | N/A       |
| `DefaultValueApplicationStrategyEnum`          | [ ] Created     | N/A       |
| `AggregationTailoring`                         | [ ] Created     | N/A       |
| `ReferenceTailoring`                           | [ ] Created     | N/A       |
| `ConstraintTailoring`                          | [ ] Created     | N/A       |
| `SdgTailoring`                                 | [ ] Created     | N/A       |
