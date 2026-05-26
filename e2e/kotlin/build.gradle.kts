import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    kotlin("jvm") version "2.3.21"
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
    testImplementation(files("../../packages/kotlin/build/libs/spikard-0.15.6-rc.6.jar"))
    testImplementation("net.java.dev.jna:jna:5.18.1")
    testImplementation("com.fasterxml.jackson.core:jackson-annotations:2.18.2")
    testImplementation("com.fasterxml.jackson.core:jackson-databind:2.18.2")
    testImplementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.18.2")
    testImplementation("org.jspecify:jspecify:1.0.0")
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")
    testImplementation("org.junit.jupiter:junit-jupiter-api:6.1.0")
    testImplementation("org.junit.jupiter:junit-jupiter-engine:6.1.0")
    testImplementation("org.junit.platform:junit-platform-launcher:6.1.0")
    testImplementation("com.fasterxml.jackson.core:jackson-databind:2.18.2")
    testImplementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.18.2")
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
    val libPath = System.getProperty("native.lib.path") ?: "${rootDir}/../../target/release"
    systemProperty("java.library.path", libPath)
    systemProperty("jna.library.path", libPath)
    // Resolve fixture paths (e.g. "docx/fake.docx") against test_documents/.
    workingDir = file("${rootDir}/../../test_documents")
}
