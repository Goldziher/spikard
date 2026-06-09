import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    kotlin("jvm") version "2.2.0"
    java
}

group = "dev.spikard"
version = "0.1.0"

java {
    sourceCompatibility = JavaVersion.VERSION_21
    targetCompatibility = JavaVersion.VERSION_21
}

kotlin {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_21)
    }
}

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(files("../../packages/kotlin/build/libs/spikard-0.15.6-rc.18.jar"))
    testImplementation("net.java.dev.jna:jna:5.18.1")
    testImplementation("com.fasterxml.jackson.core:jackson-annotations:2.18.2")
    testImplementation("com.fasterxml.jackson.core:jackson-databind:2.18.2")
    testImplementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.18.2")
    testImplementation("org.jspecify:jspecify:1.0.0")
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")
    testImplementation("org.junit.jupiter:junit-jupiter-api:6.1.0")
    testImplementation("org.junit.jupiter:junit-jupiter-engine:6.1.0")
    testImplementation("com.fasterxml.jackson.core:jackson-databind:2.18.2")
    testImplementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.18.2")
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
    val libPath = System.getProperty("native.lib.path") ?: "${rootDir}/../../target/release"
    systemProperty("java.library.path", libPath)
    systemProperty("jna.library.path", libPath)
    // Panama FFI bindings are compiled with --enable-preview against the
    // java.lang.foreign API, so the forked test worker must enable preview +
    // native access — otherwise the worker JVM aborts before JUnit starts and
    // Gradle reports a misleading "Gradle Test Executor N ... not in started or
    // detached state". Mirrors the Maven surefire argLine.
    jvmArgs("--enable-preview", "--enable-native-access=ALL-UNNAMED")
    // Resolve fixture paths (e.g. "docx/fake.docx") against test_documents/ when
    // the consumer ships such fixtures. Guard on existence: Gradle test workers
    // fail to fork if workingDir points at a directory that does not exist.
    val testDocuments = file("${rootDir}/../../test_documents")
    if (testDocuments.isDirectory) {
        workingDir = testDocuments
    }
}
